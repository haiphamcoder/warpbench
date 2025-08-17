use crate::{Error, Result};
use rhai::{AST, Engine, Map};
use std::fs;

pub struct ScriptEngine {
    engine: Engine,
    ast: Option<AST>,
}

impl ScriptEngine {
    pub fn new() -> Self {
        let mut engine = Engine::new();

        // Register types and functions
        engine.register_type::<Request>();
        engine.register_type::<Response>();

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

        Ok(())
    }

    pub fn prepare_request(&self) -> Option<Request> {
        if let Some(ast) = &self.ast {
            let mut scope = rhai::Scope::new();
            if let Ok(result) = self.engine.eval_ast_with_scope::<Request>(&mut scope, ast) {
                Some(result)
            } else {
                None
            }
        } else {
            Some(Request::default())
        }
    }

    pub fn process_response(&self, response: Response) -> Result<()> {
        if let Some(ast) = &self.ast {
            let mut scope = rhai::Scope::new();
            scope.push("response", response);

            self.engine
                .eval_ast_with_scope(&mut scope, ast)
                .map_err(|e| Error::Script(format!("Script execution error: {}", e)))?;
        }
        Ok(())
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
