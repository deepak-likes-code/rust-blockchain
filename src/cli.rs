use crate::errors::Result;
use crate::transaction::Transaction;
use crate::{blockchain::Blockchain, wallet::Wallets};
use clap::{arg, Command};
use colored::Colorize;
use std::process::exit;

pub struct Cli {}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {})
    }
    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust-demo")
            .version("0.1")
            .author("deepakkomma@gmail.com")
            .about("blockchain in rust: a simple blockchain for learning")
            .subcommand(Command::new("printchain").about("print all the chain blocks"))
            .subcommand(
                Command::new("getbalance")
                    .about("get balance in the blochain")
                    .arg(arg!(<ADDRESS>"'The Address it get balance for'")),
            )
            .subcommand(
                Command::new("create")
                    .about("Create new blochain")
                    .arg(arg!(<ADDRESS>"'The address to send gensis block reqward to' ")),
            )
            .subcommand(
                Command::new("send")
                    .about("send  in the blockchain")
                    .arg(arg!(<FROM>" 'Source wallet address'"))
                    .arg(arg!(<TO>" 'Destination wallet address'"))
                    .arg(arg!(<AMOUNT>" 'Destination wallet address'")),
            )
            .subcommand(Command::new("createwallet"))
            .about("Creates a wallet")
            .subcommand(Command::new("listaddresses"))
            .about("Lists addresses")
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("create") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                Blockchain::create_blockchain(address.clone())?;
                println!("create blockchain");
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("getbalance") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                let bc = Blockchain::new()?;
                let utxos = bc.find_UTXO(&address);
                let mut balance = 0;
                for out in utxos {
                    balance += out.value;
                }
                println!("Balance of '{}';{}", address, balance)
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("send") {
            let from = if let Some(address) = matches.get_one::<String>("FROM") {
                address
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let to = if let Some(address) = matches.get_one::<String>("TO") {
                address
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let amount: i32 = if let Some(amount) = matches.get_one::<String>("AMOUNT") {
                amount.parse()?
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let mut b = Blockchain::new()?;
            let tx = Transaction::new_UTXO(from, to, amount, &b)?;
            b.add_block(vec![tx])?;
            println!("Success");
        }

        if let Some(_) = matches.subcommand_matches("printchain") {
            cmd_print_chain()?;
        }

        if let Some(_) = matches.subcommand_matches("createwallet") {
            let mut ws = Wallets::new()?;
            let address = ws.create_wallet();
            ws.save_all()?;
            println!("success:address {}", address)
        }

        if let Some(_) = matches.subcommand_matches("listaddresses") {
            let ws = Wallets::new()?;
            let addresses = ws.get_all_addresses();
            println!("Addresses: ....");
            for a in addresses {
                println!("{}", a.green().bold())
            }
        }

        Ok(())
    }
}

fn cmd_print_chain() -> Result<()> {
    let bc = Blockchain::new()?;
    for b in bc.iter() {
        println!("{:#?}", b);
    }
    Ok(())
}
