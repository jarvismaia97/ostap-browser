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

#[tauri::command]
fn navigate_tab(app: tauri::AppHandle, url: String, tab_id: String, area: BrowseArea) -> Result<(), String> {
    let label = format!("browse-{}", tab_id);
    
    // If window already exists, just navigate it
    if let Some(window) = app.get_webview_window(&label) {
        window.navigate(url.parse().map_err(|e: url::ParseError| e.to_string())?)
            .map_err(|e| e.to_string())?;
        // Reposition in case layout changed
        let _ = window.set_position(tauri::LogicalPosition::new(area.x, area.y));
        let _ = window.set_size(tauri::LogicalSize::new(area.width, area.height));
        let _ = window.show();
        return Ok(());
    }

    // Get main window position to calculate absolute position
    let main = app.get_webview_window("main").ok_or("No main window")?;
    let main_pos = main.outer_position().map_err(|e| e.to_string())?;
    
    // Account for titlebar height (~28px on macOS)
    let titlebar_h = 28.0;
    let abs_x = main_pos.x as f64 + area.x;
    let abs_y = main_pos.y as f64 + area.y + titlebar_h;

    println!("navigate_tab: main_pos=({},{}) area=({},{},{},{}) abs=({},{})", 
        main_pos.x, main_pos.y, area.x, area.y, area.width, area.height, abs_x, abs_y);

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
    .always_on_top(false)
    .skip_taskbar(true)
    .visible(true)
    .focused(false)
    .on_page_load(move |wv, payload| {
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

    // Set parent window
    builder = builder.parent(&main).map_err(|e| e.to_string())?;

    let window = builder.build().map_err(|e| e.to_string())?;

    // Set dark appearance on the WKWebView
    #[cfg(target_os = "macos")]
    {
        let _ = window.with_webview(|platform_wv| {
            use objc::{msg_send, sel, sel_impl, class};
            unsafe {
                let wk_webview: *mut objc::runtime::Object = platform_wv.inner() as _;
                let dark_name: *mut objc::runtime::Object = msg_send![class!(NSString),
                    stringWithUTF8String: "NSAppearanceNameDarkAqua\0".as_ptr()];
                let appearance: *mut objc::runtime::Object = msg_send![class!(NSAppearance), appearanceNamed: dark_name];
                let _: () = msg_send![wk_webview, setAppearance: appearance];
            }
        });
    }

    Ok(())
}

#[tauri::command]
fn resize_tab(app: tauri::AppHandle, tab_id: String, area: BrowseArea) -> Result<(), String> {
    let label = format!("browse-{}", tab_id);
    if let Some(window) = app.get_webview_window(&label) {
        let main = app.get_webview_window("main").ok_or("No main window")?;
        let main_pos = main.outer_position().map_err(|e| e.to_string())?;
        let titlebar_h = 28.0;
        let abs_x = main_pos.x as f64 + area.x;
        let abs_y = main_pos.y as f64 + area.y + titlebar_h;
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
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_theme(Some(tauri::Theme::Dark));
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![navigate_tab, resize_tab, hide_all_tabs, close_tab_webview])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
