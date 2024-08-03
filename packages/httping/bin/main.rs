use std::io::stdout;

mod app;

use app::App;
use httping::Itdog;
use httping::Ui;

fn main() -> color_eyre::Result<()> {
    let mut ui = Ui::new(stdout())?;
    let mut app = App::default().with_server(Itdog);

    ui.run_loop(&mut app, App::view, App::update)?;

    Ok(())
}
