use chrono::{DateTime, Utc};
use ogonek_types::AuditLogCreate;
use serde_json::Value;
use sqlx::types::ipnetwork::IpNetwork;

use crate::services::{Claims, RequestMetadata};
/// Fluent builder for audit logs
pub struct AuditBuilder {
    event_type: String,
    action: String,
    outcome: String,
    user_id: Option<String>,
    user_email: String,
    user_role: String,
    impersonated_by: Option<String>,
    resource_type: String,
    resource_id: Option<String>,
    resource_name: Option<String>,
    payload: Value,
    session_id: Option<String>,
    ip_address: Option<IpNetwork>,
    user_agent: Option<String>,
    request_id: Option<String>,
    severity: String,
    tags: Vec<String>,
    retention_until: Option<DateTime<Utc>>,
}

impl AuditBuilder {
    /// The bare minimum
    pub fn new(event_type: &str, action: &str, claims: &Claims, user_email: String) -> Self {
        Self {
            event_type: event_type.to_string(),
            action: action.to_string(),
            outcome: "success".to_string(), // optimistic by default
            user_id: Some(claims.sub.clone()),
            user_email,
            user_role: claims.role.to_string(),
            impersonated_by: None,
            resource_type: "unknown".to_string(), // you'll probably want to set this
            resource_id: None,
            resource_name: None,
            payload: serde_json::json!({}),
            session_id: None,
            ip_address: None,
            user_agent: None,
            request_id: None,
            severity: "info".to_string(),
            tags: Vec::new(),
            retention_until: None,
        }
    }

    /// Quick constructor for user operations - covers 80% of use cases
    pub fn user_operation(action: &str, claims: &Claims, user_email: String) -> Self {
        Self::new("user.operation", action, claims, user_email)
            .resource_type("user")
            .tags(vec!["user_management".to_string()])
    }

    /// Set outcome (success/failure/pending)
    pub fn outcome(mut self, outcome: &str) -> Self {
        self.outcome = outcome.to_string();
        self
    }

    /// Mark as failed and bump severity to warning
    pub fn failed(mut self) -> Self {
        self.outcome = "failure".to_string();
        self.severity = "warning".to_string();
        self
    }

    /// Set resource details
    pub fn resource_type(mut self, resource_type: &str) -> Self {
        self.resource_type = resource_type.to_string();
        self
    }

    pub fn resource_id(mut self, id: String) -> Self {
        self.resource_id = Some(id);
        self
    }

    pub fn resource_name(mut self, name: String) -> Self {
        self.resource_name = Some(name);
        self
    }

    /// Set the payload - accepts anything serializable
    pub fn payload<T: serde::Serialize>(mut self, payload: T) -> Self {
        self.payload = serde_json::to_value(payload).unwrap_or_else(|_| serde_json::json!({}));
        self
    }

    /// Add request metadata in one go
    pub fn with_metadata(mut self, metadata: &RequestMetadata) -> Self {
        self.session_id = metadata.session_id.clone();
        self.ip_address = metadata.ip_address;
        self.user_agent = Some(metadata.user_agent.clone());
        self.request_id = Some(metadata.request_id.clone());
        self
    }

    /// Set severity level
    pub fn severity(mut self, severity: &str) -> Self {
        self.severity = severity.to_string();
        self
    }

    /// Add tags
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Add a single tag
    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    /// Security-related operation - adds security tag and bumps to warning
    pub fn security_event(mut self) -> Self {
        self.severity = "warning".to_string();
        self.tags.push("security".to_string());
        self
    }

    /// Build the final AuditLogCreate
    pub fn build(self) -> AuditLogCreate {
        AuditLogCreate {
            event_type: self.event_type,
            action: self.action,
            outcome: Some(self.outcome),
            user_id: self.user_id,
            user_email: self.user_email,
            user_role: self.user_role,
            impersonated_by: self.impersonated_by,
            resource_type: self.resource_type,
            resource_id: self.resource_id,
            resource_name: self.resource_name,
            payload: Some(self.payload),
            session_id: self.session_id,
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            request_id: self.request_id,
            severity: Some(self.severity),
            tags: Some(self.tags),
            retention_until: self.retention_until,
        }
    }
}

/// Convenience macro for quick audit logging
#[macro_export]
macro_rules! audit {
    ($db:expr, $event:expr, $action:expr, $claims:expr, $email:expr) => {
        $crate::audit::AuditBuilder::new($event, $action, $claims, $email).build()
    };
}

/// Convenience macro for user operations
#[macro_export]
macro_rules! audit_user {
    ($db:expr, $action:expr, $claims:expr, $email:expr) => {
        $crate::audit::AuditBuilder::user_operation($action, $claims, $email).build()
    };
}
