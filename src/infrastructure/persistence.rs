use crate::domain::repository::Persistence;
use crate::domain::App;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub struct TomlPersistence;

impl TomlPersistence {
    fn get_file_path() -> Result<PathBuf> {
        let dirs = ProjectDirs::from("com", "giks", "pomodog")
            .context("Could not find project directories")?;

        let config_dir = dirs.config_dir();
        fs::create_dir_all(config_dir)?;

        Ok(config_dir.join("session.toml"))
    }
}

impl Persistence for TomlPersistence {
    fn save(&self, app: &App) -> Result<()> {
        let path = Self::get_file_path()?;
        let toml = toml::to_string(app)?;
        fs::write(path, toml)?;
        Ok(())
    }

    fn load(&self) -> Result<Option<App>> {
        let path = Self::get_file_path()?;
        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(path)?;
        let mut app: App = toml::from_str(&content)?;

        if let Some(session) = app.take_session() {
            app.set_resume_session(session);
        }

        Ok(Some(app))
    }

    fn delete_session(&self) -> Result<()> {
        let path = Self::get_file_path()?;
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}
