use crate::components::App;
use anyhow::Error;
use ratatui::DefaultTerminal;

mod components;

fn main() -> Result<(), Error> {
    let app = App::new(10, 20, 2)?;
    let terminal = ratatui::init();
    // let app_result = app.run(&mut terminal);
    let app_result = run_app(app, terminal);
    ratatui::restore();
    app_result
}

fn run_app(mut app: App, mut terminal: DefaultTerminal) -> Result<(), Error> {
    while !app.exit() {
        terminal.draw(|frame| app.draw(frame))?;
        app = app.handle_events()?;
    }
    Ok(())
}
