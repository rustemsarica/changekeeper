use crate::models::Config;
use anyhow::Result;
use directories::{BaseDirs, ProjectDirs};
use std::fs;
use std::path::{PathBuf};

const QUALIFIER: &str = "io";
const ORGANIZATION: &str = "changekeeper";
const APPLICATION: &str = "changekeeper";

pub struct InitResult {
    pub config: Config,
    pub config_file: PathBuf,
    pub created: bool,
}

pub fn init() -> Result<InitResult> {
    let project_dirs = ProjectDirs::from(
        QUALIFIER,
        ORGANIZATION,
        APPLICATION,
    )
    .expect("unable to determine configuration directory");

    let config_dir = project_dirs.config_dir();
    fs::create_dir_all(config_dir)?;

    let config_file = config_dir.join("config.toml");

    let base_dirs = BaseDirs::new().expect("unable to determine home directory");

    let storage = base_dirs.home_dir().join("WorkChanges");
    fs::create_dir_all(&storage)?;

    let config = Config {
        version: 1,
        storage,
    };

    let created = if config_file.exists() {
        false
    } else {
        let toml = toml::to_string_pretty(&config)?;
        fs::write(&config_file, toml)?;
        true
    };

    Ok(InitResult {
        config,
        config_file,
        created,
    })
}

pub fn config_file() -> Result<PathBuf> {
    let project_dirs = ProjectDirs::from(
        QUALIFIER,
        ORGANIZATION,
        APPLICATION,
    )
    .expect("unable to determine configuration directory");

    Ok(project_dirs.config_dir().join("config.toml"))
}

pub fn load() -> Result<Config> {
    let path = config_file()?;

    let text = fs::read_to_string(path)?;

    Ok(toml::from_str(&text)?)
}