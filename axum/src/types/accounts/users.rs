use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use utoipa::ToSchema;
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdate {
    pub name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub pass: Option<String>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub pass: String,
    pub role: UserRole,
}
#[derive(sqlx::Type, Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[sqlx(rename_all = "snake_case")]
pub enum UserRole {
    Student,
    Teacher,
    Moderator,
    Admin,
    God,
}

impl UserRole {
    pub fn can_sign_up(&self) -> bool {
        matches!(self, Self::Student | Self::Teacher)
    }
}

impl User {
    pub fn is_staff(&self) -> bool {
        matches!(
            self.role,
            UserRole::Moderator | UserRole::Admin | UserRole::God
        )
    }
}
impl FromStr for UserRole {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "student" => Ok(UserRole::Student),
            "teacher" => Ok(UserRole::Teacher),
            "moderator" => Ok(UserRole::Moderator),
            "admin" => Ok(UserRole::Admin),
            "god" => Ok(UserRole::God),
            _ => Err(anyhow::anyhow!("Invalid user role: {}", s)),
        }
    }
}

impl From<String> for UserRole {
    fn from(s: String) -> Self {
        s.parse().unwrap_or(UserRole::Student)
    }
}

impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Student => write!(f, "student"),
            UserRole::Teacher => write!(f, "teacher"),
            UserRole::Moderator => write!(f, "moderator"),
            UserRole::Admin => write!(f, "admin"),
            UserRole::God => write!(f, "god"),
        }
    }
}
