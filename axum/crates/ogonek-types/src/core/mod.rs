mod files;
mod flashcards;
mod lessons;
mod tasks;
use core::fmt;

pub use files::*;
pub use flashcards::*;
pub use lessons::*;
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
pub use tasks::*;
use utoipa::ToSchema;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "varchar")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    #[default]
    Private,
    Shared,
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Visibility::Public => write!(f, "public"),
            Visibility::Private => write!(f, "private"),
            Visibility::Shared => write!(f, "shared"),
        }
    }
}

impl std::str::FromStr for Visibility {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "public" => Ok(Visibility::Public),
            "private" => Ok(Visibility::Private),
            "shared" => Ok(Visibility::Shared),
            _ => Err(format!("Invalid visibility value: {}", s)),
        }
    }
}
impl From<String> for Visibility {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

impl From<&str> for Visibility {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "public" => Visibility::Public,
            "private" => Visibility::Private,
            "shared" => Visibility::Shared,
            _ => Visibility::Private, // fallback to default
        }
    }
}
