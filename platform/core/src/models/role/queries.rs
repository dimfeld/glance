#![allow(unused_imports, unused_variables, dead_code)]
use std::str::FromStr;

use error_stack::ResultExt;
use filigree::{
    auth::{AuthInfo as _, ObjectPermission},
    errors::OrderByError,
    sql::{BindingOperator, FilterBuilder, ValuesBuilder},
};
use serde::Deserialize;
use sqlx::{
    postgres::PgRow, query_file, query_file_as, query_file_scalar, PgConnection, PgExecutor,
};
use tracing::{event, instrument, Level};

use super::{types::*, RoleId};
use crate::{auth::AuthInfo, models::organization::OrganizationId, Error};

#[derive(Debug, Default)]
enum OrderByField {
    UpdatedAt,
    CreatedAt,
    #[default]
    Name,
}

impl OrderByField {
    fn as_str(&self) -> &str {
        match self {
            Self::UpdatedAt => "updated_at",
            Self::CreatedAt => "created_at",
            Self::Name => "name",
        }
    }

    fn allowed_direction(&self, descending: bool) -> bool {
        match self {
            _ => true,
        }
    }
}

impl std::str::FromStr for OrderByField {
    type Err = OrderByError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match s {
            "updated_at" => OrderByField::UpdatedAt,
            "created_at" => OrderByField::CreatedAt,
            "name" => OrderByField::Name,
            _ => return Err(OrderByError::InvalidField),
        };

        Ok(value)
    }
}

fn parse_order_by(field: &str) -> Result<(bool, OrderByField), OrderByError> {
    let descending = field.starts_with('-');
    let field = if descending { &field[1..] } else { field };

    let value = OrderByField::from_str(field)?;
    if !value.allowed_direction(descending) {
        return Err(OrderByError::InvalidDirection);
    }
    Ok((descending, value))
}

#[derive(Deserialize, Debug, Default)]
pub struct ListQueryFilters {
    pub page: Option<u32>,
    pub per_page: Option<u32>,

    pub order_by: Option<String>,
    #[serde(default)]
    pub id: Vec<RoleId>,
    pub updated_at_lte: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at_gte: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at_lte: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at_gte: Option<chrono::DateTime<chrono::Utc>>,
}

impl ListQueryFilters {
    fn build_where_clause(&self) -> String {
        let mut bindings = FilterBuilder::new(4);

        if !self.id.is_empty() {
            bindings.add_vec("id", &self.id);
        }

        if self.updated_at_lte.is_some() {
            bindings.add_option("updated_at", &self.updated_at_lte, BindingOperator::Lte);
        }

        if self.updated_at_gte.is_some() {
            bindings.add_option("updated_at", &self.updated_at_gte, BindingOperator::Gte);
        }

        if self.created_at_lte.is_some() {
            bindings.add_option("created_at", &self.created_at_lte, BindingOperator::Lte);
        }

        if self.created_at_gte.is_some() {
            bindings.add_option("created_at", &self.created_at_gte, BindingOperator::Gte);
        }

        let query = bindings.to_string();
        event!(Level::DEBUG, %query);
        query
    }

    fn bind_to_query<'a, T>(&'a self, mut query: QueryAs<'a, T>) -> QueryAs<'a, T> {
        if !self.id.is_empty() {
            event!(Level::DEBUG, id = ?self.id);
            query = query.bind(&self.id);
        }

        if self.updated_at_lte.is_some() {
            event!(Level::DEBUG, updated_at_lte = ?self.updated_at_lte);
            query = query.bind(&self.updated_at_lte);
        }

        if self.updated_at_gte.is_some() {
            event!(Level::DEBUG, updated_at_gte = ?self.updated_at_gte);
            query = query.bind(&self.updated_at_gte);
        }

        if self.created_at_lte.is_some() {
            event!(Level::DEBUG, created_at_lte = ?self.created_at_lte);
            query = query.bind(&self.created_at_lte);
        }

        if self.created_at_gte.is_some() {
            event!(Level::DEBUG, created_at_gte = ?self.created_at_gte);
            query = query.bind(&self.created_at_gte);
        }

        query
    }
}

type QueryAs<'q, T> =
    sqlx::query::QueryAs<'q, sqlx::Postgres, T, <sqlx::Postgres as sqlx::Database>::Arguments<'q>>;

impl Role {
    /// Get a Role from the database
    #[instrument(skip(db))]
    pub async fn get(
        db: impl PgExecutor<'_>,
        auth: &AuthInfo,
        id: &RoleId,
    ) -> Result<Role, error_stack::Report<Error>> {
        auth.require_permission(super::READ_PERMISSION)?;

        let object = query_file_as!(
            Role,
            "src/models/role/select_one.sql",
            id.as_uuid(),
            auth.organization_id.as_uuid()
        )
        .fetch_optional(db)
        .await
        .change_context(Error::Db)?
        .ok_or(Error::NotFound("Role"))?;

        Ok(object)
    }

    #[instrument(skip(db))]
    pub async fn list(
        db: impl PgExecutor<'_>,
        auth: &AuthInfo,
        filters: &ListQueryFilters,
    ) -> Result<Vec<RoleListResult>, error_stack::Report<Error>> {
        let q = include_str!("list.sql");
        Self::list_internal(q, db, auth, filters).await
    }

    async fn list_internal<T>(
        query_template: &str,
        db: impl PgExecutor<'_>,
        auth: &AuthInfo,
        filters: &ListQueryFilters,
    ) -> Result<Vec<T>, error_stack::Report<Error>>
    where
        T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin,
    {
        auth.require_permission(super::READ_PERMISSION)?;

        const MAX_PER_PAGE: u32 = 200;
        const DEFAULT_PER_PAGE: u32 = 50;
        let per_page = filters
            .per_page
            .unwrap_or(DEFAULT_PER_PAGE)
            .min(MAX_PER_PAGE)
            .max(1) as i32;
        let offset = filters.page.unwrap_or(0) as i32 * per_page;
        event!(Level::DEBUG, per_page, offset);

        let (descending, order_by_field) =
            parse_order_by(filters.order_by.as_deref().unwrap_or("name"))
                .change_context(Error::Filter)?;
        let order_direction = if descending { "DESC" } else { "ASC" };

        let q = query_template.replace(
            "__insertion_point_order_by",
            &format!("{} {}", order_by_field.as_str(), order_direction),
        );

        let q = q.replace("__insertion_point_filters", &filters.build_where_clause());

        let mut query = sqlx::query_as::<_, T>(q.as_str());

        event!(Level::DEBUG, organization_id=%auth.organization_id);
        query = query
            .bind(&auth.organization_id)
            .bind(per_page)
            .bind(offset);

        query = filters.bind_to_query(query);

        let results = query.fetch_all(db).await.change_context(Error::Db)?;

        Ok(results)
    }

    /// Create a new Role in the database.
    pub async fn create(
        db: &mut PgConnection,
        auth: &AuthInfo,
        payload: RoleCreatePayload,
    ) -> Result<RoleCreateResult, error_stack::Report<Error>> {
        auth.require_permission(super::CREATE_PERMISSION)?;

        let id = RoleId::new();

        Self::create_raw(&mut *db, &id, &auth.organization_id, payload).await
    }

    /// Create a new Role in the database, allowing the ID to be explicitly specified
    /// regardless of whether it would normally be allowed.
    #[instrument(skip(db))]
    pub async fn create_raw(
        db: &mut PgConnection,
        id: &RoleId,
        organization_id: &OrganizationId,
        payload: RoleCreatePayload,
    ) -> Result<RoleCreateResult, error_stack::Report<Error>> {
        let result = query_file_as!(
            Role,
            "src/models/role/insert.sql",
            id.as_uuid(),
            organization_id.as_uuid(),
            &payload.name as _,
            payload.description.as_ref() as _
        )
        .fetch_one(&mut *db)
        .await
        .change_context(Error::Db)?;

        Ok(result)
    }

    #[instrument(skip(db))]
    pub async fn update(
        db: &mut PgConnection,
        auth: &AuthInfo,
        id: &RoleId,
        payload: RoleUpdatePayload,
    ) -> Result<bool, error_stack::Report<Error>> {
        auth.require_permission(super::WRITE_PERMISSION)?;

        let result = query_file_scalar!(
            "src/models/role/update.sql",
            &payload.name as _,
            payload.description.as_ref() as _,
            id.as_uuid(),
            auth.organization_id.as_uuid()
        )
        .execute(&mut *db)
        .await
        .change_context(Error::Db)?;

        if result.rows_affected() == 0 {
            return Ok(false);
        }

        Ok(true)
    }

    #[instrument(skip(db))]
    pub async fn delete(
        db: impl PgExecutor<'_>,
        auth: &AuthInfo,
        id: &RoleId,
    ) -> Result<bool, error_stack::Report<Error>> {
        auth.require_permission(super::CREATE_PERMISSION)?;

        let result = query_file!(
            "src/models/role/delete.sql",
            id.as_uuid(),
            auth.organization_id.as_uuid()
        )
        .execute(db)
        .await
        .change_context(Error::Db)?;
        Ok(result.rows_affected() > 0)
    }

    #[instrument(skip(db))]
    pub async fn lookup_object_permissions(
        db: impl PgExecutor<'_>,
        auth: &AuthInfo,
        #[allow(unused_variables)] id: &RoleId,
    ) -> Result<Option<ObjectPermission>, error_stack::Report<Error>> {
        let mut saw_write = false;
        let mut saw_read = false;

        use super::{OWNER_PERMISSION, READ_PERMISSION, WRITE_PERMISSION};

        for perm in &auth.permissions {
            if perm == OWNER_PERMISSION {
                return Ok(Some(ObjectPermission::Owner));
            } else if perm == WRITE_PERMISSION {
                saw_write = true;
            } else if perm == READ_PERMISSION {
                saw_read = true;
            }
        }

        if saw_write {
            return Ok(Some(ObjectPermission::Write));
        } else if saw_read {
            return Ok(Some(ObjectPermission::Read));
        } else {
            return Ok(None);
        }
    }
}
