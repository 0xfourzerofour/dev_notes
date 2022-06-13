use directories_next::ProjectDirs;
use lazy_static::lazy_static;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::{app::AppResult, projects::Project};

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

pub fn load_projects() -> Result<Vec<Project>, io::Error> {
    if !Path::new(PROJ_PATHS.data_path.as_path()).exists() {
        fs::File::create(PROJ_PATHS.data_path.as_path())?;
    }

    let stringified_projects = fs::read_to_string(PROJ_PATHS.data_path.as_path())?;
    let projects = serde_json::from_str(&stringified_projects).unwrap_or_default();

    Ok(projects)
}

pub fn save_projects(projects: &Vec<Project>) -> AppResult<()> {
    let projs = Project::new(projects);

    let stringified_projects = serde_json::to_string(projs)?;
    fs::write(PROJ_PATHS.data_path.as_path(), stringified_projects)?;
    Ok(())
}

// Load configs from the file and returns it, if there's no config set, returns default config
// pub fn load_configs() -> Result<Configs, io::Error> {
//     if !Path::new(PROJ_PATHS.config_path.as_path()).exists() {
//         save_configs(&Configs::default()).unwrap();
//     }

//     let stringified_configs = fs::read_to_string(PROJ_PATHS.config_path.as_path())?;
//     let configs: Configs = serde_json::from_str(&stringified_configs).unwrap();

//     Ok(configs)
// }

// /// Save configs to file
// fn save_configs(configs: &Configs) -> DynResult {
//     let stringified_configs = serde_json::to_string_pretty(configs)?;
//     fs::write(PROJ_PATHS.config_path.as_path(), stringified_configs)?;

//     Ok(())
// }
