use crate::{Error, Result};
use rhai::{Dynamic, Engine, Map, Scope, AST};
use std::fs;
use tracing::debug;

pub struct ScriptEngine {
    engine: Engine,
    ast: Option<AST>,
}

impl ScriptEngine {
    pub fn new() -> Self {
        let mut engine = Engine::new();

        // Register custom types
        engine.register_type::<Request>();
        engine.register_type::<Response>();

        // Register HTTP functions
        engine.register_fn("http_get", http_get);
        engine.register_fn("http_post", http_post);
        engine.register_fn("http_put", http_put);
        engine.register_fn("http_delete", http_delete);
        engine.register_fn("set_header", set_header);
        engine.register_fn("set_body", set_body);
        engine.register_fn("get_status", get_status);
        engine.register_fn("get_body", get_body);
        engine.register_fn("get_header", get_header);
        engine.register_fn("log_metric", log_metric);

        // Register utility functions
        engine.register_fn("print", |msg: &str| println!("{}", msg));
        engine.register_fn("debug", |msg: &str| debug!("Script: {}", msg));

        Self { engine, ast: None }
    }

    pub fn load_script(&mut self, path: &str) -> Result<()> {
        let script = fs::read_to_string(path)
            .map_err(|e| Error::Script(format!("Failed to read script: {}", e)))?;

        self.ast = Some(
            self.engine
                .compile(&script)
                .map_err(|e| Error::Script(format!("Failed to compile script: {}", e)))?,
        );

        debug!("Script loaded successfully from: {}", path);
        Ok(())
    }

    pub fn prepare_request(&self) -> Option<Request> {
        if let Some(ast) = &self.ast {
            let mut scope = Scope::new();
            scope.push("request", Request::default());

            // Try to call the setup function if it exists
            if let Ok(_) = self.engine.eval_ast_with_scope::<()>(&mut scope, ast) {
                // Check if the script modified the request
                if let Some(request) = scope.get_value::<Request>("request") {
                    return Some(request);
                }
            }

            // Try to call a request() function if it exists
            match self.engine.call_fn::<Request>(&mut scope, ast, "request", ()) {
                Ok(request) => Some(request),
                Err(_) => {
                    debug!("No request() function found in script, using default request");
                    None
                }
            }
        } else {
            None
        }
    }

    pub async fn process_response(&self, response: Response) -> Result<()> {
        if let Some(ast) = &self.ast {
            let mut scope = Scope::new();
            scope.push("response", response);

            // Try to call response processing function
            if let Err(e) = self.engine.call_fn::<()>(&mut scope, ast, "response", ()) {
                debug!("No response() function found in script or execution failed: {}", e);
            }

            // Execute any global script code
            let _ = self.engine
                .eval_ast_with_scope::<Dynamic>(&mut scope, ast)
                .map_err(|e| Error::Script(format!("Script execution error: {}", e)))?;
        }
        Ok(())
    }

    pub fn validate_script(&self) -> Result<()> {
        if let Some(ast) = &self.ast {
            let mut scope = Scope::new();
            scope.push("request", Request::default());
            scope.push("response", Response::default());

            // Try to compile and validate without executing
            match self.engine.eval_ast_with_scope::<Dynamic>(&mut scope, ast) {
                Ok(_) => {
                    debug!("Script validation successful");
                    Ok(())
                }
                Err(e) => Err(Error::Script(format!("Script validation failed: {}", e))),
            }
        } else {
            Err(Error::Script("No script loaded".to_string()))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub headers: Map,
    pub body: Option<String>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            method: "GET".into(),
            headers: Map::new(),
            body: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub headers: Map,
    pub body: String,
    pub latency_us: u64,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: 200,
            headers: Map::new(),
            body: String::new(),
            latency_us: 0,
        }
    }
}

// HTTP helper functions for scripts
fn http_get(_path: &str) -> Request {
    let mut request = Request::default();
    request.method = "GET".to_string();
    // Note: In a real implementation, you might want to modify the URL path here
    request
}

fn http_post(_path: &str, body: &str) -> Request {
    let mut request = Request::default();
    request.method = "POST".to_string();
    request.body = Some(body.to_string());
    request
}

fn http_put(_path: &str, body: &str) -> Request {
    let mut request = Request::default();
    request.method = "PUT".to_string();
    request.body = Some(body.to_string());
    request
}

fn http_delete(_path: &str) -> Request {
    let mut request = Request::default();
    request.method = "DELETE".to_string();
    request
}

fn set_header(request: &mut Request, name: &str, value: &str) {
    request.headers.insert(name.into(), value.into());
}

fn set_body(request: &mut Request, body: &str) {
    request.body = Some(body.to_string());
}

fn get_status(response: &Response) -> i64 {
    response.status as i64
}

fn get_body(response: &Response) -> String {
    response.body.clone()
}

fn get_header(response: &Response, name: &str) -> String {
    response
        .headers
        .get(name)
        .and_then(|v| v.clone().try_cast::<String>())
        .unwrap_or_default()
}

fn log_metric(name: &str, value: f64) {
    debug!("Custom metric - {}: {}", name, value);
    // In a real implementation, you would send this to a metrics collector
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_script_engine_creation() {
        let engine = ScriptEngine::new();
        assert!(engine.ast.is_none());
    }

    #[test]
    fn test_load_simple_script() -> Result<()> {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"
            fn request() {{
                let req = http_get("/test");
                set_header(req, "User-Agent", "WarpBench");
                req
            }}
        "#).unwrap();

        let mut engine = ScriptEngine::new();
        engine.load_script(temp_file.path().to_str().unwrap())?;
        assert!(engine.ast.is_some());

        Ok(())
    }

    #[test]
    fn test_prepare_request_without_script() {
        let engine = ScriptEngine::new();
        let request = engine.prepare_request();
        assert!(request.is_none());
    }

    #[tokio::test]
    async fn test_process_response() -> Result<()> {
        let engine = ScriptEngine::new();
        let response = Response {
            status: 200,
            headers: Map::new(),
            body: "test".to_string(),
            latency_us: 1000,
        };

        engine.process_response(response).await?;
        Ok(())
    }
}
