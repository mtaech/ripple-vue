use crate::common::env_info::EnvInfo;
use crate::{util, ApiResult};
use tauri::command;

#[command]
pub fn get_env_info() -> ApiResult<EnvInfo> {
    ApiResult::success(Some(util::env::get_env_info()))
}
