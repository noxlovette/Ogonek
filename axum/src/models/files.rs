use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

#[serde_with::serde_as]
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
    pub visibility: String,
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "Rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileSmall {
    pub id: String,
    pub name: String,
    pub s3_key: String,
    pub mime_type: Option<String>,
    pub size: i64,
    pub owner_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileMinimal {
    pub id: String,
    pub s3_key: Option<String>,
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
