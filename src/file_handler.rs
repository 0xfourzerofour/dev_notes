use directories_next::ProjectDirs;
use lazy_static::lazy_static;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::{app::AppResult, state::ProjectState};

/// Application data and config files path
struct ProjPaths {
    data_path: PathBuf,
    config_path: PathBuf,
}

lazy_static! {
    static ref PROJ_PATHS: ProjPaths = {
        let proj_dirs = ProjectDirs::from("", "", "dev_notes").unwrap();
        fs::create_dir_all(proj_dirs.data_dir()).unwrap();
        fs::create_dir_all(proj_dirs.config_dir()).unwrap();

        let data_path = proj_dirs.data_dir().join("data.json");
        let config_path = proj_dirs.config_dir().join("config.json");

        ProjPaths {
            data_path,
            config_path,
        }
    };
}

pub fn load_projects() -> Result<ProjectState, io::Error> {
    if !Path::new(PROJ_PATHS.data_path.as_path()).exists() {
        fs::File::create(PROJ_PATHS.data_path.as_path())?;
    }

    let stringified_projects = fs::read_to_string(PROJ_PATHS.data_path.as_path())?;
    let projects = serde_json::from_str(&stringified_projects).unwrap_or_default();

    Ok(projects)
}

pub fn save_projects(projects: &ProjectState) -> AppResult<()> {
    let stringified_projects = serde_json::to_string(projects)?;
    fs::write(PROJ_PATHS.data_path.as_path(), stringified_projects)?;
    Ok(())
}
