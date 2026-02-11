use tauri::Manager;

#[tauri::command]
fn open_url(app: tauri::AppHandle, url: String, tab_id: String) -> Result<(), String> {
    let label = format!("browse-{}", tab_id);
    
    // If window exists, just navigate it
    if let Some(window) = app.get_webview_window(&label) {
        window.navigate(url.parse().map_err(|e: url::ParseError| e.to_string())?)
            .map_err(|e| e.to_string())?;
        let _ = window.set_focus();
        return Ok(());
    }

    // Get main window position/size for reference
    let main_window = app.get_webview_window("main").ok_or("No main window")?;
    let pos = main_window.outer_position().map_err(|e| e.to_string())?;
    let size = main_window.outer_size().map_err(|e| e.to_string())?;

    // Create browsing window offset slightly
    let _window = tauri::WebviewWindowBuilder::new(
        &app,
        &label,
        tauri::WebviewUrl::External(url.parse().map_err(|e: url::ParseError| e.to_string())?),
    )
    .title(url.clone())
    .position(pos.x as f64 + 50.0, pos.y as f64 + 50.0)
    .inner_size(size.width as f64 * 0.9, size.height as f64 * 0.9)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn close_browse_window(app: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    let label = format!("browse-{}", tab_id);
    if let Some(window) = app.get_webview_window(&label) {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_url, close_browse_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
