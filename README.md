# Groth16 verifier on Casper

This project is an implementation of the Groth16 zk-SNARK proving system on Casper.

The project is consist of:

- An on-chain proof verifier contract
- A circuit demo
- A client can send proof and input to verifier program

### Build the on-chain contract

```
make prepare
make build
```

### Build and run the client

```
cd client
cargo build
cargo run
```
