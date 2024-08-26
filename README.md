# rust-vanity
<<<<<<< HEAD
base58 vanity search
=======

`rust-vanity` is a Rust-based Bitcoin vanity address generator. This tool generates Bitcoin Pay-to-PubKey-Hash (P2PKH) addresses with a specific prefix. It uses multi-threading to speed up the search for the desired address.

## Features

- Generates Bitcoin P2PKH addresses.
- Allows you to specify a prefix for the generated address.
- Uses multi-threading to speed up the address search process.
- Outputs the corresponding private key, public key, and address.

## Prerequisites

To build and run `rust-vanity`, you need to have Rust installed. If Rust is not installed on your system, you can install it using the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

##  Installation

1.Clone the repository:

```bash
git clone https://github.com/nzengi/rust-vanity.git
cd rust-vanity
```
2. Build the project in release mode:

```bash
cargo build --release
```
##  Usage

```bash
cargo run --release -- <prefix>
```

##  Example Output

```bash
private_key: Kx5JfiAGuEzkz9aZp1yMPycfNkvh7f29yV8eZ3jMBShDEfY5MjW8
public_key: 03a34f805d45a18443d74eaf24835b4e8bb6e0b5b07bc1fbb5e43b2990d5ebc7bb
Address: 1abcD4YjhdqF2v3KJgkE7ZJKVFnw7P2JkL
```
