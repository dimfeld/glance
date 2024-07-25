#![allow(unused_imports, dead_code)]
use filigree::auth::ObjectPermission;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sqlx_transparent_json_decode::sqlx_json_decode;

use super::RoleId;
use crate::models::organization::OrganizationId;

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow, Serialize)]
pub struct Role {
    pub id: RoleId,
    pub organization_id: crate::models::organization::OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub description: Option<String>,
}

pub type RoleListResult = Role;

pub type RolePopulatedGetResult = Role;

pub type RolePopulatedListResult = Role;

pub type RoleCreateResult = Role;

impl Role {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> RoleId {
        <RoleId as Default>::default().into()
    }

    pub fn default_organization_id() -> crate::models::organization::OrganizationId {
        <crate::models::organization::OrganizationId as Default>::default().into()
    }

    pub fn default_updated_at() -> chrono::DateTime<chrono::Utc> {
        <chrono::DateTime<chrono::Utc> as Default>::default().into()
    }

    pub fn default_created_at() -> chrono::DateTime<chrono::Utc> {
        <chrono::DateTime<chrono::Utc> as Default>::default().into()
    }

    pub fn default_name() -> String {
        <String as Default>::default().into()
    }

    pub fn default_description() -> Option<String> {
        None
    }
}

sqlx_json_decode!(Role);

impl Default for Role {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            organization_id: Self::default_organization_id(),
            updated_at: Self::default_updated_at(),
            created_at: Self::default_created_at(),
            name: Self::default_name(),
            description: Self::default_description(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow)]
#[cfg_attr(test, derive(Serialize))]
pub struct RoleCreatePayloadAndUpdatePayload {
    pub id: Option<RoleId>,
    pub name: String,
    pub description: Option<String>,
}

pub type RoleCreatePayload = RoleCreatePayloadAndUpdatePayload;

pub type RoleUpdatePayload = RoleCreatePayloadAndUpdatePayload;

impl RoleCreatePayloadAndUpdatePayload {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> Option<RoleId> {
        None
    }

    pub fn default_name() -> String {
        <String as Default>::default().into()
    }

    pub fn default_description() -> Option<String> {
        None
    }
}

impl Default for RoleCreatePayloadAndUpdatePayload {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            name: Self::default_name(),
            description: Self::default_description(),
        }
    }
}
