#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .register_uri_scheme_protocol("fileserver", |app, request| {
            tauri::http::Response::builder()
                .status(tauri::http::StatusCode::OK)
                .header("Access-Control-Allow-Origin", "*")
                .body("<h1>hello</h1>".as_bytes())
                .unwrap()
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
