mod client;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    client::run()?;

    Ok(())
}
