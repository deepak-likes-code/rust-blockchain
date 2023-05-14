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

Currently, this project contains a test module that demonstrates the basic functionality of the blockchain.

You can run this test using the command:

```bash
cargo test
```

This will create a new blockchain, add a few blocks to it, and print out the blockchain's state.

## Understanding the Code

Here is a brief overview of the important parts of the code:

- `Block`: This struct represents a block in the blockchain. Each block contains data (in the form of a string), a timestamp, a nonce, a hash, the hash of the previous block, and its height in the blockchain.

- `Blockchain`: This struct represents the blockchain itself, which is a list of blocks.

- `Block::new_block`: This function is used to create a new block. It computes the block's hash using a simple proof-of-work mechanism.

- `Block::proof_of_work`: This function attempts to find a nonce such that the hash of the block's data, the nonce, and some other information starts with a certain number of zeroes.

- `Blockchain::new`: This function creates a new blockchain, starting with a genesis block.

- `Blockchain::add_block`: This function adds a block with given data to the blockchain.

The `tests` module contains a simple test that demonstrates creating a blockchain and adding blocks to it.
