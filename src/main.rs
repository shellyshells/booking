// =============================================================================
// ROOM RESERVATION SYSTEM - Main Entry Point
// =============================================================================

mod models;
mod patterns;
mod services;

use patterns::singleton::CONFIG;
use services::AppState;
use std::fs;
use std::io::Read;
use std::sync::Arc;
use tiny_http::{Header, Method, Request, Response, Server};

fn main() {
    let config = CONFIG.get();
    let bind_address = format!("{}:{}", config.server_host, config.server_port);
    let app_state = Arc::new(AppState::new());

    let server = Server::http(&bind_address).expect("Failed to start server");

    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║     ROOM RESERVATION SYSTEM - Design Patterns Demo            ║");
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║  Server running at: http://{}                    ║", bind_address);
    println!("║  Web Interface: http://{}                        ║", bind_address);
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    for request in server.incoming_requests() {
        let state = app_state.clone();
        handle_request(request, state);
    }
}

fn handle_request(mut request: Request, state: Arc<AppState>) {
    let url = request.url().to_string();
    let method = request.method().clone();

    // Handle POST routes that need body
    if method == Method::Post && url == "/api/rooms" {
        let response = handle_create_room(&mut request, &state);
        let _ = request.respond(response);
        return;
    }
    
    if method == Method::Post && url == "/api/reservations" {
        let response = handle_create_reservation(&mut request, &state);
        let _ = request.respond(response);
        return;
    }

    let response = match (&method, url.as_str()) {
        (Method::Get, "/api/health") => json_response(&serde_json::json!({
            "status": "healthy",
            "version": "1.0.0"
        })),
        
        (Method::Get, "/api/rooms") => {
            let rooms = state.list_rooms();
            json_response(&serde_json::json!({"success": true, "data": rooms}))
        }
        
        (Method::Get, "/api/reservations") => {
            let reservations = state.list_reservations(None);
            json_response(&serde_json::json!({"success": true, "data": reservations}))
        }
        
        (Method::Get, "/api/statistics") => {
            let stats = state.get_statistics();
            json_response(&serde_json::json!({"success": true, "data": stats}))
        }
        
        (Method::Get, "/api/room-types") => {
            let types = state.get_room_types();
            json_response(&serde_json::json!({"success": true, "data": types}))
        }
        
        (Method::Get, "/api/validation-strategies") => {
            let strategies = state.get_validation_strategies();
            json_response(&serde_json::json!({"success": true, "data": strategies}))
        }
        
        (Method::Get, "/api/observers") => {
            let observers = state.get_observers();
            json_response(&serde_json::json!({"success": true, "data": observers}))
        }
        
        (Method::Get, "/api/logs") => {
            let logs = state.get_logs(100);
            json_response(&serde_json::json!({"success": true, "data": logs}))
        }
        
        (Method::Get, "/api/config") => {
            let config = state.get_config();
            json_response(&serde_json::json!({"success": true, "data": config}))
        }
        
        (Method::Post, path) if path.starts_with("/api/reservations/") && path.ends_with("/cancel") => {
            let id = path.trim_start_matches("/api/reservations/").trim_end_matches("/cancel");
            match state.cancel_reservation(id) {
                Ok(res) => json_response(&serde_json::json!({"success": true, "data": res})),
                Err(e) => json_response(&serde_json::json!({"success": false, "error": e}))
            }
        }
        
        (Method::Post, path) if path.starts_with("/api/reservations/") && path.ends_with("/confirm") => {
            let id = path.trim_start_matches("/api/reservations/").trim_end_matches("/confirm");
            match state.confirm_reservation(id) {
                Ok(res) => json_response(&serde_json::json!({"success": true, "data": res})),
                Err(e) => json_response(&serde_json::json!({"success": false, "error": e}))
            }
        }
        
        (Method::Post, path) if path.starts_with("/api/reservations/") && path.ends_with("/checkin") => {
            let id = path.trim_start_matches("/api/reservations/").trim_end_matches("/checkin");
            match state.check_in(id) {
                Ok(res) => json_response(&serde_json::json!({"success": true, "data": res})),
                Err(e) => json_response(&serde_json::json!({"success": false, "error": e}))
            }
        }
        
        (Method::Get, "/") | (Method::Get, "/index.html") => serve_file("static/index.html", "text/html"),
        (Method::Get, "/css/styles.css") => serve_file("static/css/styles.css", "text/css"),
        (Method::Get, "/js/app.js") => serve_file("static/js/app.js", "application/javascript"),
        
        _ => Response::from_string(r#"{"success": false, "error": "Not Found"}"#)
            .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
            .with_status_code(404)
    };

    let _ = request.respond(response);
}

fn json_response<T: serde::Serialize>(data: &T) -> Response<std::io::Cursor<Vec<u8>>> {
    let json = serde_json::to_string(data).unwrap_or_else(|_| "{}".to_string());
    Response::from_string(json)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
        .with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
}

fn serve_file(path: &str, content_type: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    match fs::read_to_string(path) {
        Ok(content) => Response::from_string(content)
            .with_header(Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap()),
        Err(_) => Response::from_string("File not found").with_status_code(404)
    }
}

fn handle_create_room(request: &mut Request, state: &AppState) -> Response<std::io::Cursor<Vec<u8>>> {
    let mut body = String::new();
    if request.as_reader().read_to_string(&mut body).is_err() {
        return json_response(&serde_json::json!({"success": false, "error": "Failed to read body"}));
    }

    match serde_json::from_str::<models::CreateRoomRequest>(&body) {
        Ok(req) => match state.create_room(req) {
            Ok(room) => json_response(&serde_json::json!({"success": true, "data": room})),
            Err(e) => json_response(&serde_json::json!({"success": false, "error": e}))
        },
        Err(e) => json_response(&serde_json::json!({"success": false, "error": format!("Invalid JSON: {}", e)}))
    }
}

fn handle_create_reservation(request: &mut Request, state: &AppState) -> Response<std::io::Cursor<Vec<u8>>> {
    let mut body = String::new();
    if request.as_reader().read_to_string(&mut body).is_err() {
        return json_response(&serde_json::json!({"success": false, "error": "Failed to read body"}));
    }

    match serde_json::from_str::<models::CreateReservationRequest>(&body) {
        Ok(req) => match state.create_reservation(req) {
            Ok(result) => json_response(&serde_json::json!({"success": true, "data": result})),
            Err(e) => json_response(&serde_json::json!({"success": false, "error": e}))
        },
        Err(e) => json_response(&serde_json::json!({"success": false, "error": format!("Invalid JSON: {}", e)}))
    }
}
