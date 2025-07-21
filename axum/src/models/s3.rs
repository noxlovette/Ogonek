use sqlx::types::chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

pub struct MultipartInitResultS3 {
    pub upload_id: String,
    pub parts: Vec<PartUploadUrl>,
}

#[derive(Debug, Serialize)]
pub struct FileMetadata {
    pub id: String,
    pub name: String,
    pub s3_key: String,
    pub path: String,
    pub mime_type: Option<String>,
    pub size: i64,
    pub is_folder: bool,
    pub parent_id: Option<String>,
    pub owner_id: String,
    pub visibility: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitUploadRequest {
    pub file_name: String,
    pub content_type: String,
    pub file_size: i64,
    pub total_parts: i32,
    pub parent_id: Option<String>,
    pub task_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartUploadUrl {
    pub part_number: i32,
    pub url: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipartUploadInit {
    pub upload_id: String,
    pub file_id: String,
    pub s3_key: String,
    pub parts: Vec<PartUploadUrl>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteMultipartRequest {
    pub upload_id: String,
    pub file_id: String,
    pub s3_key: String,
    pub parts: Vec<CompletedPart>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbortMultipartRequest {
    pub upload_id: String,
    pub file_id: String,
    pub s3_key: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletedPart {
    pub part_number: i32,
    pub etag: String,
}
