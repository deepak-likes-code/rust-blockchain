pub mod block;
pub mod blockchain;
pub mod cli;
pub mod errors;

use errors::Result;

fn main() -> Result<()> {
    let mut cli = cli::Cli::new()?;
    cli.run()?;
    Ok(())
}
