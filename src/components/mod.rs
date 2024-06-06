use app::App;
use color_eyre::Result;

mod tui;
mod app;
mod errors;

pub(crate) fn init_ui() -> Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}