use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .register_uri_scheme_protocol("img", |app, request| {
            println!("IMAGE SERVER");
            let path = &request.uri().path()[1..]; // skip the first /
            tauri::http::Response::builder()
                .status(tauri::http::StatusCode::OK)
                .header("Access-Control-Allow-Origin", "*")
                .body(std::fs::read("./huakunshen.png").unwrap())
                .unwrap()
        })
        .register_uri_scheme_protocol("fileserver", |app, request| {
            tauri::http::Response::builder()
                .status(tauri::http::StatusCode::OK)
                .header("Access-Control-Allow-Origin", "*")
                .body("<h1>hello</h1>".as_bytes())
                .unwrap()
        })
        .setup(|app| {
            // open dev tools in dev mode
            #[cfg(debug_assertions)]
            app.get_webview_window("main").unwrap().open_devtools();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
