use tauri::{Emitter, Manager, Window};
use tauri_plugin_updater::UpdaterExt;

#[tauri::command]
fn send_url(url: &str) -> String {
    format!("URL received: {}", url)
}

#[tauri::command]
fn set_current_url(url: &str) -> String {
    format!("Current URL set to: {}", url)
}

#[tauri::command]
async fn go_to_translate(window: Window) {
    if let Err(e) = window.emit("go-to-translate", ()) {
        eprintln!("Failed to send go-to-translate event: {}", e);
    }
}

#[tauri::command]
async fn show_window(window: Window) {
    if let Err(e) = window.show() {
        eprintln!("Failed to show window: {}", e);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = update(handle).await {
                    eprintln!("Failed to check for updates: {}", e);
                }
            });

            let quit = tauri::menu::MenuItemBuilder::with_id("quit", "Quit")
                .accelerator("CmdOrCtrl+Q")
                .build(app)
                .unwrap();
            let show = tauri::menu::MenuItemBuilder::with_id("show", "Show")
                .build(app)
                .unwrap();
            let hide = tauri::menu::MenuItemBuilder::with_id("hide", "Hide")
                .build(app)
                .unwrap();

            let menu = tauri::menu::MenuBuilder::new(app)
                .items(&[&show, &hide, &quit])
                .build()
                .unwrap();

            let _tray = tauri::tray::TrayIconBuilder::new()
                .tooltip("Kagi Translate")
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id().0.as_str() {
                    "quit" => app.exit(0),
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            window.hide().unwrap();
                        }
                    }
                    _ => {}
                })
                .build(app)
                .unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            send_url,
            set_current_url,
            go_to_translate,
            show_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {} from {:?}", downloaded, content_length);
                },
                || {
                    println!("download finished");
                },
            )
            .await?;

        println!("update installed");
        app.restart();
    }

    Ok(())
}
