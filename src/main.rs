pub mod block;
pub mod blockchain;
pub mod cli;
pub mod errors;
pub mod transaction;
pub mod txn;
pub mod wallet;

use errors::Result;

fn main() -> Result<()> {
    let mut cli = cli::Cli::new()?;
    cli.run()?;
    Ok(())
}
