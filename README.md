# rust-okapi-template

*TODO general description*

## Prerequisites

[Install rust and cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Run following command to enable watch mode:
```sh
cargo install cargo-watch
```

[Install visual studio code extension for rust](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

Run following command to install diesel cli:
```sh
cargo install diesel_cli --no-default-features --features postgres-bundled --features sqlite-bundle
```

## Commands

In order to start application in dev mode:
```sh
./run.sh dev
```

To run tests:
```sh
./run.sh test
```

## Recommended programs

- Visual Studio Code for file edits
- [TablePlus](https://tableplus.com) for databases
