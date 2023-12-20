use color_eyre::eyre::Result;

mod app;
mod event;
mod tui;
mod ui;
mod update;

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("Hello, world!");

    Ok(())
}
