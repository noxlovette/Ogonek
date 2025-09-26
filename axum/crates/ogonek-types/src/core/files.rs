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
#[derive(Deserialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PDFType {
    Task,
    Lesson,
}

#[derive(Deserialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PDFQuery {
    pub pdf_type: Option<PDFType>,
}

/// File creation parameters - groups all the file metadata together
#[derive(Debug, Clone)]
pub struct FileCreateParams {
    pub file_id: String,
    pub file_name: String,
    pub s3_key: String,
    pub content_type: String,
    pub file_size: i64,
    pub parent_id: Option<String>,
    pub owner_id: String,
}

/// Optional linking parameters for files
#[derive(Debug, Clone, Default)]
pub struct FileLinkOptions {
    pub task_id: Option<String>,
}
pub struct PDFData {
    pub title: String,
    pub markdown: String,
}

impl FileCreateParams {
    /// Builder pattern for cleaner construction
    pub fn new(file_id: String, file_name: String, owner_id: String) -> Self {
        Self {
            file_id,
            file_name: file_name.clone(),
            s3_key: String::new(), // Will be set later
            content_type: "application/octet-stream".to_string(),
            file_size: 0,
            parent_id: None,
            owner_id,
        }
    }

    pub fn with_s3_key(mut self, s3_key: String) -> Self {
        self.s3_key = s3_key;
        self
    }

    pub fn with_content_type(mut self, content_type: String) -> Self {
        self.content_type = content_type;
        self
    }

    pub fn with_size(mut self, file_size: i64) -> Self {
        self.file_size = file_size;
        self
    }

    pub fn with_parent(mut self, parent_id: Option<String>) -> Self {
        self.parent_id = parent_id;
        self
    }
}
