use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: String,
    pub name: String,
    pub s3_key: String,
    pub path: String,
    pub mime_type: Option<String>,
    pub size: i64,
    pub is_folder: bool,
    pub parent_id: Option<String>,
    pub owner_id: String,
    pub upload_status: Option<String>,
    pub visibility: String,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FileSmall {
    pub id: String,
    pub name: String,
    pub s3_key: Option<String>,
    pub mime_type: Option<String>,
    pub size: i64,
    pub owner_id: String,
}

#[derive(Debug)]
pub struct S3KeyRecord {
    pub s3_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileUpdate {
    pub name: Option<String>,
    pub path: Option<String>,
    pub parent_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FileListParams {
    pub parent_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UploadParams {
    pub parent_id: Option<String>,
    pub task_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderRequest {
    pub name: String,
    pub parent_id: Option<String>,
}

// Add this response schema
#[derive(Serialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PresignedUrlResponse {
    pub url: String,
}

// For batch requests, you might also want this:
#[derive(Serialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatchPresignedUrlResponse {
    pub urls: Vec<PresignedFileUrl>,
}

#[derive(Serialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PresignedFileUrl {
    pub file_id: String,
    pub url: String,
}
