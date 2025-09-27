use log::info;
use tauri_plugin_updater::UpdaterExt;

pub async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
	if let Some(update) = app.updater()?.check().await? {
		let mut downloaded = 0;

		update.download_and_install(
			|chunk_length, content_length| {
				downloaded += chunk_length;
				info!("downloaded {downloaded} from {content_length:?}");
			},
			|| {
				info!("download finished");
			}
		).await?;

		info!("update installed");
		app.restart();
	}

	Ok(())
}
