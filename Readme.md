# Simple Blockchain in Rust

This is a basic implementation of a blockchain in Rust. It showcases the basic principles of blockchain: creating blocks with data, adding them to the blockchain, and ensuring the integrity and validity of the chain through a simple proof-of-work mechanism.

## Installation

1. First, make sure you have Rust installed on your machine. If you don't have Rust installed, you can install it using `rustup`. Instructions for installing `rustup` can be found on the [official Rust website](https://www.rust-lang.org/tools/install).

2. Clone the repository to your local machine:

```bash
git clone https://github.com/deepak-likes-code/rust-blockchain
```

3. Navigate to the project directory and build the project:

```bash
cd rust_blockchain
cargo build
```

## Usage

The entry point for this application is the `cargo run` command.

## Commands

The following commands are available in the CLI:

1. **printchain**: This command prints all the chain blocks. Usage:

   ```
   cargo run  printchain
   ```

2. **getbalance**: This command retrieves the balance of a given address in the blockchain. Usage:

   ```
   cargo run  getbalance [ADDRESS]
   ```

   Replace `[ADDRESS]` with the address you want to retrieve the balance for.

3. **create**: This command creates a new blockchain and sends the genesis block reward to the specified address. Usage:

   ```
   cargo run  create [ADDRESS]
   ```

   Replace `[ADDRESS]` with the address where you want to send the genesis block reward.

4. **send**: This command sends an amount from one wallet to another in the blockchain. Usage:

   ```
   cargo run  send [FROM] [TO] [AMOUNT]
   ```

   Replace `[FROM]` with the source wallet address, `[TO]` with the destination wallet address, and `[AMOUNT]` with the amount to send.

5. **createwallet**: This command creates a new wallet. Usage:

   ```
   cargo run  createwallet
   ```

6. **listaddresses**: This command lists all wallet addresses. Usage:

   ```
   cargo run  listaddresses
   ```

7. **reindex**: This command reindexes the UTXOSet. Usage:

   ```
   cargo run  reindex
   ```

## Error Handling

If an error occurs while executing any of the commands, the program will display a descriptive error message and exit with status 1.

## Understanding the Code

Here is a brief overview of the important parts of the code:

- `Block`: This struct represents a block in the blockchain. Each block contains data (in the form of a string), a timestamp, a nonce, a hash, the hash of the previous block, and its height in the blockchain.

- `Blockchain`: This struct represents the blockchain itself, which is a list of blocks.

- `Block::new_block`: This function is used to create a new block. It computes the block's hash using a simple proof-of-work mechanism.

- `Block::proof_of_work`: This function attempts to find a nonce such that the hash of the block's data, the nonce, and some other information starts with a certain number of zeroes.

- `Blockchain::new`: This function creates a new blockchain, starting with a genesis block.

- `Blockchain::add_block`: This function adds a block with given data to the blockchain.

The `tests` module contains a simple test that demonstrates creating a blockchain and adding blocks to it.

## Author

Deepak Komma (deepakkomma@gmail.com)

## Version

0.1

-

This README is always evolving. If you think it's missing something crucial, please don't hesitate to suggest improvements.
