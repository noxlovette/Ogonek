use crate::db::crud::core::task::{delete_system, fetch_old_tasks};
use crate::error::AppError;
use crate::schema::AppState;

use crate::db::crud::core::files::file::fetch_files_task;
pub async fn daily_cleanup(state: AppState) {
    loop {
        tracing::info!("Starting daily cleanup job...");

        match fetch_old_tasks(&state.db).await {
            Ok(tasks) => {
                for task in tasks {
                    if let Err(e) = cleanup_task(&state, task.clone()).await {
                        tracing::error!("Failed to clean up task {}: {:?}", task, e);
                    }
                }
            }
            Err(e) => {
                tracing::error!("Failed to fetch old tasks: {:?}", e);
            }
        }

        // Sleep for 24 hours
        tokio::time::sleep(std::time::Duration::from_secs(60 * 60 * 24)).await;
    }
}

async fn cleanup_task(state: &AppState, id: String) -> Result<(), AppError> {
    let files = fetch_files_task(&state.db, &id).await?;

    let file_ids: Vec<String> = files.iter().map(|f| f.id.clone()).collect();

    delete_system(&state.db, &id, file_ids).await?;

    for file in files {
        if let Some(s3_key) = file.s3_key
            && let Err(e) = state.s3.delete_s3(&s3_key).await
        {
            tracing::error!("Failed to delete file from S3: {}, error: {:?}", s3_key, e);
        }
    }

    Ok(())
}
