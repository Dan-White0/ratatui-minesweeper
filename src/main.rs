use crate::components::App;
use anyhow::Error;

mod components;

fn main() -> Result<(), Error> {
    let mut terminal = ratatui::init();
    let app_result = App::new(10, 10, 10)?.run(&mut terminal);
    ratatui::restore();
    app_result
}
