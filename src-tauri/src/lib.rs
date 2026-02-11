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
}

#[tauri::command]
fn navigate_tab(app: tauri::AppHandle, url: String, tab_id: String, area: BrowseArea) -> Result<(), String> {
    let label = format!("browse-{}", tab_id);
    
    if let Some(webview) = app.get_webview(&label) {
        webview.navigate(url.parse().map_err(|e: url::ParseError| e.to_string())?)
            .map_err(|e| e.to_string())?;
        return Ok(());
    }

    let window = app.get_window("main").ok_or("No main window")?;

    let app_handle = app.clone();
    let tid = tab_id.clone();
    // Add dark mode param for Google
    let mut final_url = url.clone();
    if final_url.contains("google.com") && !final_url.contains("dark=1") {
        let sep = if final_url.contains('?') { "&" } else { "?" };
        final_url = format!("{}{}cs=1", final_url, sep);
    }

    let webview_builder = WebviewBuilder::new(
        &label,
        WebviewUrl::External(final_url.parse().map_err(|e: url::ParseError| e.to_string())?),
    )
    .on_page_load(move |wv, payload| {
        if let tauri::webview::PageLoadEvent::Finished = payload.event() {
            let _ = app_handle.emit("tab-updated", TabUpdate {
                tab_id: tid.clone(),
                url: payload.url().to_string(),
            });
            // Inject dark theme
            let _ = wv.eval(r#"
                (function() {
                    if (document.getElementById('ostap-dark')) return;
                    // Force dark color scheme
                    document.documentElement.style.colorScheme = 'dark';
                    var m = document.querySelector('meta[name="color-scheme"]');
                    if (m) m.content = 'dark';
                    else {
                        m = document.createElement('meta');
                        m.name = 'color-scheme';
                        m.content = 'dark';
                        document.head.appendChild(m);
                    }
                    // Fallback: invert for sites without native dark mode
                    var s = document.createElement('style');
                    s.id = 'ostap-dark';
                    s.textContent = `
                        :root { color-scheme: dark !important; }
                        @media not (prefers-color-scheme: dark) {
                            html {
                                filter: invert(0.9) hue-rotate(180deg) !important;
                                background: #0a0a0a !important;
                            }
                            img, video, canvas, svg, picture,
                            [style*="background-image"], iframe {
                                filter: invert(1) hue-rotate(180deg) !important;
                            }
                        }
                    `;
                    document.head.appendChild(s);
                })();
            "#);
        }
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
fn show_tab(app: tauri::AppHandle, tab_id: String, area: BrowseArea) -> Result<(), String> {
    let label = format!("browse-{}", tab_id);
    
    for l in app.webview_windows().keys() {
        if l.starts_with("browse-") && *l != label {
            if let Some(wv) = app.get_webview(l) {
                let _ = wv.set_position(tauri::LogicalPosition::new(-9999.0, -9999.0));
            }
        }
    }
    
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
