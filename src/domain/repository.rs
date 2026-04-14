use crate::domain::App;
use anyhow::Result;

pub trait Persistence: Send + Sync {
    fn save(&self, app: &App) -> Result<()>;
    fn load(&self) -> Result<Option<App>>;
    fn delete_session(&self) -> Result<()>;
}
