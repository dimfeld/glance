use std::{path::Path, sync::Arc};

use effectum::Queue;
use error_stack::{Report, ResultExt};
use filigree::{
    auth::password::HashedPassword,
    users::roles::{add_permissions_to_role, add_roles_to_user},
};
use glance_app::{AppData, AppItemData, Notification};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, PgExecutor, PgPool};
use tracing::instrument;

use crate::{
    items::{AppInfo, AppItems, Item},
    models::{
        organization::OrganizationId,
        role::{self, RoleId},
        user::{UserCreatePayload, UserId},
    },
    scheduled_task::ScheduledJobData,
    users::{
        organization::create_new_organization, users::create_new_user_with_prehashed_password,
    },
    Error,
};

/// Run the database migrations, if needed
pub async fn run_migrations(db: &PgPool) -> Result<(), Report<Error>> {
    sqlx::migrate!().run(db).await.change_context(Error::Db)
}

/// Arguments to the [boostrap] function.
#[derive(Debug, Default)]
pub struct BootstrapData {
    /// If false, don't do anything if the database already contains at least one organization.
    /// If true, try to add the admin user and organization regardless of what's in the database
    /// right now.
    pub force: bool,
    /// The email for the admin user
    pub admin_email: String,
    /// The name of the admin user, or "Admin" if omitted
    pub admin_name: Option<String>,
    /// The hashed password for the admin user. Can be omitted if you only want to do passwordless
    /// login methods.
    pub admin_password: Option<HashedPassword>,
    /// The name of the administrator's organization.
    pub organization_name: Option<String>,
}

/// Bootstrap the database, adding an administrator user and organization.
/// This users gets the special superuser role, which has a "_global:admin" permission.
pub async fn bootstrap(db: PgPool, data: BootstrapData) -> Result<bool, Report<Error>> {
    let mut tx = db.begin().await.unwrap();

    if !data.force {
        let any_exists = sqlx::query_scalar!("SELECT true FROM organizations LIMIT 1")
            .fetch_optional(&mut *tx)
            .await
            .change_context(Error::Db)?
            .is_some();

        if any_exists {
            return Ok(false);
        }
    }

    let admin_user_id = UserId::new();

    let org = create_new_organization(
        &mut *tx,
        data.organization_name
            .unwrap_or_else(|| "Administration".to_string()),
        admin_user_id,
    )
    .await?;

    let user_details = UserCreatePayload {
        name: data.admin_name.unwrap_or_else(|| "Admin".to_string()),
        email: Some(data.admin_email),
        ..Default::default()
    };

    create_new_user_with_prehashed_password(
        &mut *tx,
        admin_user_id,
        org.organization.id,
        user_details,
        data.admin_password,
    )
    .await?;
    let superuser_role = create_superuser_role(&mut *tx, org.organization.id).await?;

    add_roles_to_user(
        &mut *tx,
        org.organization.id,
        admin_user_id,
        &[org.admin_role, org.user_role, superuser_role],
    )
    .await
    .change_context(Error::Db)?;

    Ok(true)
}

async fn create_superuser_role(
    tx: &mut PgConnection,
    org_id: OrganizationId,
) -> Result<RoleId, Error> {
    let superuser_role_id = RoleId::new();
    let superuser_role = role::RoleCreatePayload {
        name: "Superuser".to_string(),
        description: None,
    };

    add_permissions_to_role(
        &mut *tx,
        org_id,
        superuser_role_id,
        &["_global:admin".to_string()],
    )
    .await
    .change_context(Error::Db)?;

    role::queries::create_raw(tx, superuser_role_id, org_id, &superuser_role).await?;

    Ok(superuser_role_id)
}

/// The database for the glance platform
pub struct DbInner {
    /// The database connection pool
    pub pool: sqlx::PgPool,
    pub(crate) task_queue: Queue,
}

impl std::fmt::Debug for DbInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DbInner").finish_non_exhaustive()
    }
}

/// The database for the glance platform, wrapped in an [Arc].
pub type Db = Arc<DbInner>;

/// Event types that can be recorded
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    /// An item was created for the first time
    CreateItem,
    /// An existing item was updated
    UpdateItem,
    /// An item was removed
    RemoveItem,
    /// An app and its items were removed
    RemoveApp,
    /// A scheduled app was executed
    ScheduledRun,
}

impl DbInner {
    /// Create a new database connection and run migrations if needed.
    pub async fn new(pool: PgPool, data_dir: &Path) -> Result<Self, Report<Error>> {
        let task_queue = Queue::new(&data_dir.join("glance_tasks.db"))
            .await
            .change_context(Error::DbInit)?;

        Ok(Self { pool, task_queue })
    }

    #[instrument(skip(self))]
    async fn add_event(
        &self,
        tx: &mut PgConnection,
        event_type: EventType,
        app_id: &str,
        item_id: Option<&str>,
        data: Option<serde_json::Value>,
    ) -> Result<(), Report<Error>> {
        sqlx::query_file!("src/add_event.sql", event_type as _, app_id, item_id, data)
            .execute(tx)
            .await
            .change_context(Error::Db)?;
        Ok(())
    }

    /// Record an error reading the data for an app.
    #[instrument(skip(self))]
    pub async fn update_app_status(
        &self,
        conn: impl PgExecutor<'_>,
        app_id: &str,
        error: Option<&str>,
    ) -> Result<(), Report<Error>> {
        sqlx::query!(
            r##"UPDATE apps SET updated_at = now(), error = $2 WHERE id = $1"##,
            app_id,
            error
        )
        .execute(conn)
        .await
        .change_context(Error::Db)?;
        Ok(())
    }

    /// List all the known apps
    #[instrument(skip(self))]
    pub async fn get_apps(&self, app_ids: &[String]) -> Result<Vec<AppInfo>, Report<Error>> {
        sqlx::query_file_as!(AppInfo, "src/get_apps.sql", app_ids)
            .fetch_all(&self.pool)
            .await
            .change_context(Error::Db)
    }

    /// Remove an app and all its associated items.
    #[instrument(skip(self))]
    pub async fn remove_app(&self, app_id: &str) -> Result<(), Report<Error>> {
        sqlx::query_file!("src/remove_app.sql", app_id)
            .execute(&self.pool)
            .await
            .change_context(Error::Db)?;

        let scheduled = self
            .task_queue
            .list_recurring_jobs_with_prefix(app_id)
            .await
            .change_context(Error::TaskQueue)?;
        for job in scheduled {
            let res = self.task_queue.delete_recurring_job(job).await;
            match res {
                Err(effectum::Error::NotFound) => Ok(()),
                _ => res.change_context(Error::TaskQueue),
            }?;
        }

        Ok(())
    }

    /// Update the dismissed state of an item.
    #[instrument(skip(self))]
    pub async fn set_item_dismissed(
        &self,
        app_id: &str,
        item_id: &str,
        dismissed: bool,
    ) -> Result<(), Report<Error>> {
        sqlx::query!(
            "UPDATE items SET dismissed = $3 WHERE app_id = $1 AND id = $2",
            app_id,
            item_id,
            dismissed
        )
        .execute(&self.pool)
        .await
        .change_context(Error::Db)?;
        Ok(())
    }

    /// Update an app, or create it if it doesn't exist.
    #[instrument(skip(self))]
    pub async fn create_or_update_app(
        &self,
        tx: impl PgExecutor<'_>,
        app_id: &str,
        app: &AppData,
    ) -> Result<(), Report<Error>> {
        sqlx::query_file!(
            "src/create_or_update_app.sql",
            app_id,
            app.name,
            app.path,
            sqlx::types::Json(&app.ui) as _,
        )
        .execute(tx)
        .await
        .change_context(Error::Db)?;

        let mut existing_jobs = self
            .task_queue
            .list_recurring_jobs_with_prefix(&format!("{app_id}:"))
            .await
            .change_context(Error::TaskQueue)?;

        for schedule in &app.schedule {
            let job_id = format!("{app_id}:{}", schedule.cron);
            existing_jobs.retain(|existing| existing != &job_id);
            self.task_queue
                .upsert_recurring_job(
                    job_id,
                    effectum::RecurringJobSchedule::Cron {
                        spec: schedule.cron.clone(),
                    },
                    effectum::Job::builder("scheduled-app")
                        .json_payload(&ScheduledJobData {
                            app_id: app_id.to_string(),
                            command: app.path.clone(),
                            schedule: schedule.clone(),
                        })
                        .change_context(Error::TaskQueue)?
                        .timeout(std::time::Duration::from_secs(
                            schedule.timeout.unwrap_or(300) as u64,
                        ))
                        .build(),
                    false,
                )
                .await
                .change_context(Error::TaskQueue)?;
        }

        // Anything left in existing_jobs was not seen as we went through the current schedule
        // list.
        for to_remove in existing_jobs {
            self.task_queue
                .delete_recurring_job(to_remove)
                .await
                .change_context(Error::TaskQueue)?;
        }

        Ok(())
    }

    /// Update an item, or update it if an item with the same ID already exists.
    #[instrument(skip(self))]
    pub async fn create_or_update_item(
        &self,
        tx: impl PgExecutor<'_>,
        item: &Item,
        resurface: bool,
    ) -> Result<(), Report<Error>> {
        sqlx::query_file!(
            "src/create_or_update_item.sql",
            item.id,
            item.app_id,
            sqlx::types::Json(&item.data) as _,
            item.state_key,
            item.persistent,
            item.updated_at,
            resurface
        )
        .execute(tx)
        .await
        .change_context(Error::Db)?;
        Ok(())
    }

    /// Remove the items with ids that do not match the passed list
    #[instrument(skip(self))]
    pub async fn remove_unfound_items(
        &self,
        tx: &mut PgConnection,
        app_id: &str,
        item_ids: &[String],
    ) -> Result<(), Report<Error>> {
        sqlx::query!(
            "DELETE FROM items WHERE app_id = $1 AND id <> ALL($2)",
            app_id,
            item_ids
        )
        .execute(tx)
        .await
        .change_context(Error::Db)?;
        Ok(())
    }

    /// Read all the items for the given app from the database
    #[instrument(skip(self))]
    pub async fn read_app_items(&self, app_id: &str) -> Result<Vec<Item>, Report<Error>> {
        let items = sqlx::query_file_as!(Item, "src/get_items_by_app_id.sql", app_id)
            .fetch_all(&self.pool)
            .await
            .change_context(Error::Db)?;

        Ok(items)
    }

    /// Read all the non-dismissed items for all apps from the database
    #[instrument(skip(self))]
    pub async fn read_active_items(&self) -> Result<Vec<AppItems>, Report<Error>> {
        let mut items = sqlx::query_file_as!(Item, "src/get_active_items.sql")
            .fetch_all(&self.pool)
            .await
            .change_context(Error::Db)?;

        items.sort_unstable_by(|i1, i2| i1.app_id.cmp(&i2.app_id));

        let mut items_by_app_id = items
            .into_iter()
            .into_grouping_map_by(|item| item.app_id.to_string())
            .collect::<Vec<_>>();
        let app_ids = items_by_app_id.keys().cloned().collect::<Vec<_>>();
        let apps = self.get_apps(&app_ids).await?;

        let apps_with_items = apps
            .into_iter()
            .map(|app| {
                let items = items_by_app_id.remove(app.id.as_str()).unwrap_or_default();
                AppItems { app, items }
            })
            .collect::<Vec<_>>();

        Ok(apps_with_items)
    }
}
