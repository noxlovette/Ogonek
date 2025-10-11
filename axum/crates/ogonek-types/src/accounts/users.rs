use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use utoipa::ToSchema;
use validator::Validate;

use crate::CalendarRole;
#[derive(Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdate {
    #[validate(length(min = 3))]
    pub name: Option<String>,
    #[validate(length(min = 2, max = 50))]
    pub username: Option<String>,
    pub email: Option<String>,
    #[validate(length(min = 8))]
    pub pass: Option<String>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub verified: bool,
}

pub struct UserForClaims {
    pub id: String,
    pub role: UserRole,
    pub pass: String,
}

#[derive(sqlx::Type, Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Student,
    Teacher,
    Moderator,
    Admin,
    God,
}
impl From<UserRole> for CalendarRole {
    fn from(user_role: UserRole) -> Self {
        match user_role {
            UserRole::Student => CalendarRole::Student,
            UserRole::Teacher => CalendarRole::Teacher,
            UserRole::Moderator => CalendarRole::Teacher,
            UserRole::Admin => CalendarRole::Teacher,
            UserRole::God => CalendarRole::Teacher,
        }
    }
}
impl UserRole {
    /// Roles that can be self-assigned during signup
    pub fn can_self_assign(&self) -> bool {
        matches!(self, Self::Student | Self::Teacher)
    }

    /// Roles that require admin privileges to assign
    pub fn requires_admin_to_assign(&self) -> bool {
        matches!(self, Self::Moderator | Self::Admin | Self::God)
    }

    /// Who can assign this role to others
    pub fn can_be_assigned_by(&self, assigner_role: &UserRole) -> bool {
        match self {
            Self::Student | Self::Teacher => true, // Anyone can assign basic roles
            Self::Moderator => matches!(assigner_role, Self::Admin | Self::God),
            Self::Admin => matches!(assigner_role, Self::God),
            Self::God => false, // God role can't be assigned, only exists in DB
        }
    }

    pub fn hierarchy_level(&self) -> u8 {
        match self {
            Self::Student => 0,
            Self::Teacher => 1,
            Self::Moderator => 2,
            Self::Admin => 3,
            Self::God => 4,
        }
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
