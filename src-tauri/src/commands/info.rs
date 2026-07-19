use crate::{APP_BUILD_CODE, APP_BUILD_DATE, APP_PACKAGE_ID, APP_VERSION};

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct BuildInfo {
    pub version: &'static str,
    pub build_code: u32,
    pub build_date: &'static str,
    pub target_os: &'static str,
    pub package_id: &'static str,
}

#[tauri::command]
pub fn get_build_info() -> BuildInfo {
    BuildInfo {
        version: APP_VERSION,
        build_code: APP_BUILD_CODE,
        build_date: APP_BUILD_DATE,
        target_os: std::env::consts::OS,
        package_id: APP_PACKAGE_ID,
    }
}
