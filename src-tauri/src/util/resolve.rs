use crate::util::init;
use crate::DB;
use log::info;
use migration::{Migrator, MigratorTrait};
use tauri::{App, Wry};

/// handle something when start app
pub fn resolve_setup(_app: &mut App<Wry>) {
    info!("resolve_setup");
    init::init_logger().unwrap();
    init::init_attachment_dir();
    init::init_cover_file();

    tauri::async_runtime::spawn(async move {
        let db = init::init_db().await.unwrap();
        Migrator::up(&db, None).await.expect("migrator error");
        DB.set(db).unwrap();
    });
}
