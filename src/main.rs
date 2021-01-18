use anyhow::Result;

mod output;

fn main() -> Result<()> {
    output::example_png()?;
    Ok(())
}
