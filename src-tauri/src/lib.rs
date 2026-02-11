use tauri::{Emitter, Manager, WebviewBuilder, WebviewUrl};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct BrowseArea {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[derive(Clone, Serialize)]
struct TabUpdate {
    tab_id: String,
    url: String,
    title: String,
}

const DARK_THEME_JS: &str = r#"
(function() {
    if (document.getElementById('ostap-dark')) return;
    var s = document.createElement('style');
    s.id = 'ostap-dark';
    s.textContent = 'html { filter: invert(0.9) hue-rotate(180deg) !important; background: #0a0a0a !important; } img, video, canvas, picture, [style*="background-image"], embed, object { filter: invert(1) hue-rotate(180deg) !important; }';
    var target = document.head || document.documentElement;
    target.appendChild(s);
    // Also set inline style as fallback
    document.documentElement.style.setProperty('filter', 'invert(0.9) hue-rotate(180deg)', 'important');
    document.documentElement.style.setProperty('background', '#0a0a0a', 'important');
})();
"#;

#[tauri::command]
fn navigate_tab(app: tauri::AppHandle, url: String, tab_id: String, area: BrowseArea) -> Result<(), String> {
    let label = format!("browse-{}", tab_id);
    
    if let Some(webview) = app.get_webview(&label) {
        webview.navigate(url.parse().map_err(|e: url::ParseError| e.to_string())?)
            .map_err(|e| e.to_string())?;
        // Re-inject dark theme after navigation
        let _ = webview.eval(DARK_THEME_JS);
        return Ok(());
    }

    let window = app.get_window("main").ok_or("No main window")?;

    let app_handle = app.clone();
    let app_handle2 = app.clone();
    let tid = tab_id.clone();
    let tid2 = tab_id.clone();

    let webview_builder = WebviewBuilder::new(
        &label,
        WebviewUrl::External(url.parse().map_err(|e: url::ParseError| e.to_string())?),
    )
    .initialization_script(DARK_THEME_JS)
    .on_page_load(move |wv, payload| {
        match payload.event() {
            tauri::webview::PageLoadEvent::Started => {
                let _ = wv.eval(DARK_THEME_JS);
            }
            tauri::webview::PageLoadEvent::Finished => {
                let _ = app_handle.emit("tab-updated", TabUpdate {
                    tab_id: tid.clone(),
                    url: payload.url().to_string(),
                    title: String::new(),
                });
                let _ = wv.eval(DARK_THEME_JS);
            }
            _ => {}
        }
    })
    .on_document_title_changed(move |_wv, title| {
        let _ = app_handle2.emit("tab-updated", TabUpdate {
            tab_id: tid2.clone(),
            url: String::new(),
            title,
        });
    });

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
    for label in app.webview_windows().keys() {
        if label.starts_with("browse-") {
            if let Some(wv) = app.get_webview(label) {
                let _ = wv.set_position(tauri::LogicalPosition::new(-9999.0, -9999.0));
            }
        }
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
        .invoke_handler(tauri::generate_handler![navigate_tab, resize_tab, hide_all_tabs, close_tab_webview])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
