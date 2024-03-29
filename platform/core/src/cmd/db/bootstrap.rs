use clap::{Args, Subcommand};
use error_stack::{Report, ResultExt};
use sqlx::PgPool;

use crate::Error;

#[derive(Args, Debug)]
pub struct BootstrapCommand {
    /// The email for the admin user
    #[clap(env = "GLANCE_BOOTSTRAP_ADMIN_EMAIL")]
    admin_email: String,

    /// The name for the admin user
    /// Defaults to "Admin"
    #[clap(env = "GLANCE_BOOTSTRAP_ADMIN_NAME")]
    admin_name: Option<String>,

    /// The password for the admin user. If supplied, this should be prehashed
    /// by the `util hash` subcommand. If omitted, login through OAuth2 and passwordless methods
    /// will still work.
    #[clap(env = "GLANCE_BOOTSTRAP_ADMIN_PASSWORD")]
    admin_password: Option<String>,

    /// The name for the admin user's organization.
    /// Defaults to "Administration"
    #[clap(env = "GLANCE_BOOTSTRAP_ORG_NAME")]
    organization_name: Option<String>,

    /// Force adding the admin user even if the database already contains at least one
    /// organization.
    #[clap(long, env = "GLANCE_BOOTSTRAP_FORCE")]
    force: bool,
}

impl BootstrapCommand {
    pub async fn handle(self, pg_pool: PgPool) -> Result<(), Report<Error>> {
        let data = crate::db::BootstrapData {
            force: self.force,
            admin_email: self.admin_email,
            admin_name: self.admin_name,
            admin_password: self
                .admin_password
                .map(filigree::auth::password::HashedPassword),
            organization_name: self.organization_name,
        };

        let bootstrapped = crate::db::bootstrap(pg_pool, data).await?;
        if bootstrapped {
            println!("Bootstrapped database");
        } else {
            println!("Database already bootstrapped");
        }

        Ok(())
    }
}
