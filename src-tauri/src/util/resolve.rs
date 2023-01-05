use crate::util::init;
use crate::DB;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use tauri::api::private::OnceCell;
use tauri::{App, Wry};

/// handle something when start app
pub fn resolve_setup(_app: &mut App<Wry>) {
    println!("resolve_setup");
    init::init_logger().unwrap();

    tauri::async_runtime::spawn(async move {
        let db = init::init_db().await.unwrap();
        Migrator::up(&db, None).await.expect("migrator error");
        DB.set(db).unwrap();
    });
}
