// Simple Rhai script example for WarpBench
// This script demonstrates basic request customization

fn request() {
    // Create a GET request
    let req = http_get("/get");
    
    // Add custom headers
    set_header(req, "X-Test-Header", "WarpBench-Script");
    set_header(req, "X-Request-ID", `req-${rand()}`);
    
    // Return the customized request
    req
}

fn response(res) {
    // Process the response
    let status = get_status(res);
    let body = get_body(res);
    
    if status == 200 {
        print("✓ Request successful");
        
        // Log custom metrics
        log_metric("response_length", body.len());
        
        // Check for specific content
        if body.contains("httpbin") {
            log_metric("httpbin_detected", 1.0);
        }
    } else {
        print(`✗ Request failed with status: ${status}`);
    }
}
