use super::item::delegate_item;
use crate::{Result, app::App, cli::Cli, controller::init::init_db};
use sqlx::SqlitePool;

pub async fn delegate(cli: Cli) -> Result<()> {
    // Start TUI if no operation is specified
    let db = init_db().await?;

    match cli.item {
        Some(op) => delegate_item(&db, op).await.map(|_| ()),
        None => start_tui(db).await,
    }
}

pub async fn start_tui(db: SqlitePool) -> Result<()> {
    let terminal = ratatui::init();
    let app_result = App::new(db).await?.run(terminal).await;
    ratatui::restore();
    app_result
}
