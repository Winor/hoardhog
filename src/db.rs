use rocket_sync_db_pools::{database, diesel};

#[database("pgcon")]
pub struct PgDbConn(diesel::PgConnection);