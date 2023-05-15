use clap::{arg, Command};

use crate::blockchain::Blockchain;
use crate::errors::Result;
use colored::*;

pub struct Cli {
    bc: Blockchain,
}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {
            bc: Blockchain::new()?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust")
            .version("0.1")
            .author("Deepak")
            .about("A simple POW blockchain implemented in Rust")
            .subcommand(
                Command::new("addblock")
                    .about("Adds a block onto the blockchain")
                    .arg(arg!(<DATA>"'the blockchain data")),
            )
            .subcommand(
                Command::new("printchain").about("Prints the information about all the blocks"),
            )
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("addblock") {
            if let Some(c) = matches.get_one::<String>("DATA") {
                self.addblock(String::from(c))?
            } else {
                println!("Not printing testing lists....");
            }
        }

        if let Some(matches) = matches.subcommand_matches("printchain") {
            self.print_chain();
        }

        Ok(())
    }

    fn addblock(&mut self, data: String) -> Result<()> {
        self.bc.add_block(data)
    }

    fn print_chain(&mut self) {
        for block in &mut self.bc.iter() {
            println!("{}", format!("block: {:#?}", block).green())
        }
    }
}
