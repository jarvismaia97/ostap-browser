use tauri::{Manager, WebviewBuilder, WebviewUrl};
use serde::Deserialize;

#[derive(Deserialize)]
struct BrowseArea {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[tauri::command]
fn navigate_tab(app: tauri::AppHandle, url: String, tab_id: String, area: BrowseArea) -> Result<(), String> {
    let label = format!("browse-{}", tab_id);
    
    // If webview already exists, just navigate it
    if let Some(webview) = app.get_webview(&label) {
        webview.navigate(url.parse().map_err(|e: url::ParseError| e.to_string())?)
            .map_err(|e| e.to_string())?;
        return Ok(());
    }

    // Get the main Window
    let window = app.get_window("main").ok_or("No main window")?;

    // Create embedded webview inside the main window
    let webview_builder = WebviewBuilder::new(
        &label,
        WebviewUrl::External(url.parse().map_err(|e: url::ParseError| e.to_string())?),
    );

    window.add_child(
        webview_builder,
        tauri::LogicalPosition::new(area.x, area.y),
        tauri::LogicalSize::new(area.width, area.height),
    ).map_err(|e: tauri::Error| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn resize_tab(app: tauri::AppHandle, tab_id: String, area: BrowseArea) -> Result<(), String> {
    let label = format!("browse-{}", tab_id);
    if let Some(webview) = app.get_webview(&label) {
        webview.set_position(tauri::LogicalPosition::new(area.x, area.y))
            .map_err(|e| e.to_string())?;
        webview.set_size(tauri::LogicalSize::new(area.width, area.height))
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn hide_all_tabs(app: tauri::AppHandle) -> Result<(), String> {
    for (label, webview) in app.webview_windows() {
        if label.starts_with("browse-") {
            let _ = webview.set_position(tauri::LogicalPosition::new(-9999.0, -9999.0));
        }
    }
    Ok(())
}

#[tauri::command]
fn show_tab(app: tauri::AppHandle, tab_id: String, area: BrowseArea) -> Result<(), String> {
    let label = format!("browse-{}", tab_id);
    
    // Hide all browse webviews
    for (l, webview) in app.webview_windows() {
        if l.starts_with("browse-") && l != label {
            let _ = webview.set_position(tauri::LogicalPosition::new(-9999.0, -9999.0));
        }
    }
    
    // Show the requested one
    if let Some(webview) = app.get_webview(&label) {
        webview.set_position(tauri::LogicalPosition::new(area.x, area.y))
            .map_err(|e| e.to_string())?;
        webview.set_size(tauri::LogicalSize::new(area.width, area.height))
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn close_tab_webview(app: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    let label = format!("browse-{}", tab_id);
    if let Some(webview) = app.get_webview(&label) {
        webview.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![navigate_tab, resize_tab, hide_all_tabs, show_tab, close_tab_webview])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
