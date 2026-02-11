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
    
    let outer_pos = main.outer_position().map_err(|e| e.to_string())?;
    // macOS titlebar is ~28px (standard) but outer_position already includes it
    // The content area starts at outer_pos.y + titlebar_height
    let titlebar_h = 28.0; // macOS standard titlebar
    let abs_x = outer_pos.x as f64 + area.x;
    let abs_y = outer_pos.y as f64 + titlebar_h + area.y;

    println!("navigate_tab: outer=({},{}) titlebar_h={} area=({},{},{},{}) abs=({},{})", 
        outer_pos.x, outer_pos.y, titlebar_h, area.x, area.y, area.width, area.height, abs_x, abs_y);

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
    .initialization_script(r#"
        (function(){
            if(document.getElementById('od'))return;
            var s=document.createElement('style');s.id='od';
            s.textContent='html{filter:invert(.9) hue-rotate(180deg)!important;background:#0a0a0a!important}img,video,canvas,picture,[style*=background-image]{filter:invert(1) hue-rotate(180deg)!important}';
            (document.head||document.documentElement).appendChild(s);
        })();
    "#)
    .on_page_load(move |wv, payload| {
        let dark_js = "(function(){if(document.getElementById('od'))return;var s=document.createElement('style');s.id='od';s.textContent='html{filter:invert(.9) hue-rotate(180deg)!important;background:#0a0a0a!important}img,video,canvas,picture,[style*=background-image]{filter:invert(1) hue-rotate(180deg)!important}';(document.head||document.documentElement).appendChild(s)})()";
        match payload.event() {
            tauri::webview::PageLoadEvent::Started => {
                let _ = wv.eval(dark_js);
            }
            tauri::webview::PageLoadEvent::Finished => {
                let _ = app_handle.emit("tab-updated", TabUpdate {
                    tab_id: tid.clone(),
                    url: payload.url().to_string(),
                    title: String::new(),
                });
                let _ = wv.eval(dark_js);
            }
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

    // Inject dark theme via WKUserScript (native, bypasses CSP)
    #[cfg(target_os = "macos")]
    {
        use objc::{msg_send, sel, sel_impl, class};
        let _ = window.set_theme(Some(tauri::Theme::Dark));
        let _ = window.with_webview(|platform_wv| {
            unsafe {
                let wk_webview: *mut objc::runtime::Object = platform_wv.inner() as _;
                // Set dark appearance
                let dark_name: *mut objc::runtime::Object = msg_send![class!(NSString),
                    stringWithUTF8String: "NSAppearanceNameDarkAqua\0".as_ptr()];
                let appearance: *mut objc::runtime::Object = msg_send![class!(NSAppearance), appearanceNamed: dark_name];
                let _: () = msg_send![wk_webview, setAppearance: appearance];
                
                // Get WKUserContentController and inject CSS via WKUserScript
                let config: *mut objc::runtime::Object = msg_send![wk_webview, configuration];
                let controller: *mut objc::runtime::Object = msg_send![config, userContentController];
                
                let js_source: *mut objc::runtime::Object = msg_send![class!(NSString),
                    stringWithUTF8String: b"(function(){var s=document.createElement('style');s.id='od';s.textContent='html{filter:invert(.9) hue-rotate(180deg)!important;background:#0a0a0a!important}img,video,canvas,picture,[style*=background-image]{filter:invert(1) hue-rotate(180deg)!important}';(document.head||document.documentElement).appendChild(s)})()\0".as_ptr()];
                
                // WKUserScriptInjectionTimeAtDocumentStart = 0
                let alloc: *mut objc::runtime::Object = msg_send![class!(WKUserScript), alloc];
                let script: *mut objc::runtime::Object = msg_send![alloc,
                    initWithSource: js_source
                    injectionTime: 0u64
                    forMainFrameOnly: true
                ];
                
                let _: () = msg_send![controller, addUserScript: script];
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
        let outer_pos = main.outer_position().map_err(|e| e.to_string())?;
        let titlebar_h = 28.0;
        let abs_x = outer_pos.x as f64 + area.x;
        let abs_y = outer_pos.y as f64 + titlebar_h + area.y;
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
        .setup(|_app| {
            // Force dark appearance globally at the NSApp level
            // This makes ALL WKWebViews report prefers-color-scheme: dark
            #[cfg(target_os = "macos")]
            {
                use objc::{msg_send, sel, sel_impl, class};
                unsafe {
                    let nsapp: *mut objc::runtime::Object = msg_send![class!(NSApplication), sharedApplication];
                    let dark_name: *mut objc::runtime::Object = msg_send![class!(NSString),
                        stringWithUTF8String: "NSAppearanceNameDarkAqua\0".as_ptr()];
                    let appearance: *mut objc::runtime::Object = msg_send![class!(NSAppearance), appearanceNamed: dark_name];
                    let _: () = msg_send![nsapp, setAppearance: appearance];
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![navigate_tab, resize_tab, hide_all_tabs, close_tab_webview])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
