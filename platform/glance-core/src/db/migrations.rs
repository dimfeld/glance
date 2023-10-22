use rusqlite::{params, Connection};
use rusqlite_migration::{Migrations, M};

pub fn run_migrations(conn: &mut Connection) -> rusqlite_migration::Result<()> {
    let migrations = Migrations::new(vec![M::up(include_str!("migrations/00001_initial.sql"))]);

    migrations.to_latest(conn)
}
