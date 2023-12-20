use color_eyre::eyre::Result;

mod app;
mod event;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("Hello, world!");

    Ok(())
}
