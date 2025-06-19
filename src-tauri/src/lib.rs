use tauri_plugin_updater::UpdaterExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .on_window_event(|_window, event| match event {
            tauri::WindowEvent::CloseRequested { .. } => {
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
            Ok(())
        })
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