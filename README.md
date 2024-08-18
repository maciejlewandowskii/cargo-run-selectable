# cargo-run-selectable

A simple Rust utility that helps developers run `cargo run` in workspaces with multiple binary targets without needing to specify one beforehand.
When you run the program, it will prompt you to select a target from the list of available binary targets in the workspace.

## Features

- Very lightweight
- Seamless integration with Cargo workflows.
- Works out of the box

## Windows Incompatibility

**Important:** This utility is currently **incompatible with Windows** due to the lack of support for the `process::CommandExt` trait's `exec` method on the Windows platform. The `exec` method is crucial for replacing the current process with a new one, which is not natively supported in Windows. For more details on this limitation, you can refer to [this explanation](https://stackoverflow.com/a/53479765).

As a result, this utility is best suited for Unix-like operating systems such as Linux and macOS.


## Installation

You can install this utility directly from [crates.io](https://crates.io/crates/cargo-run-selectable) using `cargo` or `binstall`.
### Using `cargo install`:

```bash
cargo install cargo-run-selectable
```
### Using `cargo binstall`:

```bash
cargo binstall cargo-run-selectable
```

### Cloning from repository:

```bash
git clone https://github.com/maciejlewandowskii/cargo-run-selectable
cd cargo-run-selectable
cargo build --release # now you can copy file from target/release and just use it 
