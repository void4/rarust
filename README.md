# rarust

[rarVM](https://esolangs.org/wiki/RarVM) implementation in Rust.

## Installation
Uses the 'nightly' version of Rust. Install it with

```
rustup install nightly
```

Set it as default with

```
rustup default nightly
```

## Usage

Run it with
```
cargo run
```

## Structure

```
src
├── container.rs - wraps the vm and allows functions be attached to it that can be called from inside
├── formats.rs - defines the vm process format and (de)serialization of the root process into/from u64 vecs
├── main.rs - contains and runs an example hello world program
├── ops.rs - defines the operations the vm understands and their bytecode format, stack, gas and memory requirements
└── vm.rs - implements the recursive virtual machine
```
