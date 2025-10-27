use crate::appstate::App;
use anyhow::Error;
use ratatui::DefaultTerminal;

mod appstate;
mod components;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = App::new();
    let terminal = ratatui::init();
    let app_result = run_app(app, terminal).await;
    ratatui::restore();
    app_result
}

async fn run_app(mut app: App, mut terminal: DefaultTerminal) -> Result<(), Error> {
    while !app.exit() {
        terminal.draw(|frame| app.draw(frame))?;
        app = app.handle_events().await?;
    }
    Ok(())
}
