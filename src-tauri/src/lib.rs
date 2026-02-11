use tauri::{Emitter, Manager, WebviewWindowBuilder, WebviewUrl};
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

fn calc_abs_position(main: &tauri::WebviewWindow, area: &BrowseArea) -> Result<(f64, f64), String> {
    let outer_pos = main.outer_position().map_err(|e| e.to_string())?;
    let titlebar_h = 28.0; // macOS standard
    Ok((
        outer_pos.x as f64 + area.x,
        outer_pos.y as f64 + titlebar_h + area.y,
    ))
}

#[tauri::command]
fn navigate_tab(app: tauri::AppHandle, url: String, tab_id: String, area: BrowseArea) -> Result<(), String> {
    let label = format!("browse-{}", tab_id);
    
    if let Some(window) = app.get_webview_window(&label) {
        // If same URL, just show/reposition. Otherwise navigate.
        let current_url = window.url().map(|u| u.to_string()).unwrap_or_default();
        if current_url != url {
            window.navigate(url.parse().map_err(|e: url::ParseError| e.to_string())?)
                .map_err(|e| e.to_string())?;
        }
        let main = app.get_webview_window("main").ok_or("No main window")?;
        let (abs_x, abs_y) = calc_abs_position(&main, &area)?;
        let _ = window.set_position(tauri::LogicalPosition::new(abs_x, abs_y));
        let _ = window.set_size(tauri::LogicalSize::new(area.width, area.height));
        let _ = window.show();
        return Ok(());
    }

    let main = app.get_webview_window("main").ok_or("No main window")?;
    let (abs_x, abs_y) = calc_abs_position(&main, &area)?;

    let app_handle = app.clone();
    let app_handle2 = app.clone();
    let tid = tab_id.clone();
    let tid2 = tab_id.clone();

    let mut builder = WebviewWindowBuilder::new(
        &app,
        &label,
        WebviewUrl::External(url.parse().map_err(|e: url::ParseError| e.to_string())?),
    )
    .title("")
    .inner_size(area.width, area.height)
    .position(abs_x, abs_y)
    .decorations(false)
    .skip_taskbar(true)
    .visible(true)
    .focused(false)
    .on_page_load(move |_wv, payload| {
        if let tauri::webview::PageLoadEvent::Finished = payload.event() {
            let _ = app_handle.emit("tab-updated", TabUpdate {
                tab_id: tid.clone(),
                url: payload.url().to_string(),
                title: String::new(),
            });
        }
    })
    .on_document_title_changed(move |_wv, title| {
        let _ = app_handle2.emit("tab-updated", TabUpdate {
            tab_id: tid2.clone(),
            url: String::new(),
            title,
        });
    });

    builder = builder.parent(&main).map_err(|e| e.to_string())?;
    let _window = builder.build().map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn resize_tab(app: tauri::AppHandle, tab_id: String, area: BrowseArea) -> Result<(), String> {
    let label = format!("browse-{}", tab_id);
    if let Some(window) = app.get_webview_window(&label) {
        let main = app.get_webview_window("main").ok_or("No main window")?;
        let (abs_x, abs_y) = calc_abs_position(&main, &area)?;
        let _ = window.set_position(tauri::LogicalPosition::new(abs_x, abs_y));
        let _ = window.set_size(tauri::LogicalSize::new(area.width, area.height));
    }
    Ok(())
}

#[tauri::command]
fn hide_all_tabs(app: tauri::AppHandle) -> Result<(), String> {
    for (label, window) in app.webview_windows() {
        if label.starts_with("browse-") {
            let _ = window.hide();
        }
    }
    Ok(())
}

#[tauri::command]
fn close_tab_webview(app: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    let label = format!("browse-{}", tab_id);
    if let Some(window) = app.get_webview_window(&label) {
        let _ = window.close();
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, _event| {
                    if shortcut.key == tauri_plugin_global_shortcut::Code::KeyT {
                        let _ = app.emit("shortcut", "new-tab");
                    } else if shortcut.key == tauri_plugin_global_shortcut::Code::KeyW {
                        let _ = app.emit("shortcut", "close-tab");
                    } else if shortcut.key == tauri_plugin_global_shortcut::Code::KeyL {
                        let _ = app.emit("shortcut", "focus-url");
                    }
                })
                .build(),
        )
        .setup(|app| {
            use tauri_plugin_global_shortcut::{GlobalShortcutExt, Modifiers, Code, Shortcut};
            let cmd_t = Shortcut::new(Some(Modifiers::SUPER), Code::KeyT);
            let cmd_w = Shortcut::new(Some(Modifiers::SUPER), Code::KeyW);
            let cmd_l = Shortcut::new(Some(Modifiers::SUPER), Code::KeyL);
            let _ = app.global_shortcut().register(cmd_t);
            let _ = app.global_shortcut().register(cmd_w);
            let _ = app.global_shortcut().register(cmd_l);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![navigate_tab, resize_tab, hide_all_tabs, close_tab_webview])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
