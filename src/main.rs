use crate::components::App;
use anyhow::Error;

mod components;

fn main() -> Result<(), Error> {
    let mut app = App::new(10, 20, 20)?;
    let mut terminal = ratatui::init();
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
