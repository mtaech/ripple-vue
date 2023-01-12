use crate::util::env;
use crate::util::env::{get_attachment_dir_dir, get_config_dir_path, get_cover_dir_dir};
use log::info;
use rust_embed::RustEmbed;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(RustEmbed)]
#[folder = "asset/"]
struct Asset;

pub fn get_db_path() -> PathBuf {
    let path = get_config_dir_path().join("db");
    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
    }
    path
}

pub async fn init_db() -> Result<DatabaseConnection, DbErr> {
    let path = get_db_path();
    let path = path.join("ripple.db");
    let db_file = format!(
        "sqlite:{}?mode=rwc",
        path.into_os_string().into_string().unwrap()
    );
    info!("sqlx db path {}", db_file);
    let mut opt = ConnectOptions::new(db_file);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Warn); // Setting default PostgreSQL schema

    let db = Database::connect(opt).await?;
    Ok(db)
}

pub fn init_logger() -> Result<(), fern::InitError> {
    let log_path = env::get_log_dir_path();
    let log_file = format!("fca-{}.log", chrono::Local::now().format("%Y-%m-%d"));
    let log_path = log_path.join(log_file);
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_path)?)
        .apply()?;
    Ok(())
}

pub fn init_attachment_dir() -> PathBuf {
    get_attachment_dir_dir()
}

pub fn init_cover_file() {
    let cover_dir_path = get_cover_dir_dir();
    let cover_path = cover_dir_path.join("cover.png");
    let cover_dark_path = cover_dir_path.join("cover_dark.png");
    let cover = Asset::get("cover/cover.png").unwrap();
    let cover_dark = Asset::get("cover/cover_dark.png").unwrap();
    fs::write(cover_path, cover.data.as_ref()).expect("copy error");
    fs::write(cover_dark_path, cover_dark.data.as_ref()).expect("copy error");
}
