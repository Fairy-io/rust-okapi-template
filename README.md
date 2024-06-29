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
cargo install diesel_cli --no-default-features --features postgres-bundled --features sqlite-bundled
```

## No pq library error
You may encounter this error on MacOS. In order to fix it, run following commands:
```sh
brew install libpq
brew link --force libpq
cargo clean // optionally
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

## Recommended addons

- [Visual Studio Code](https://code.visualstudio.com/download) for file edits
- [TablePlus](https://tableplus.com) for database view
- [Docker](https://docs.docker.com/engine/install/) to set up database with command `docker-compose up -d`
- [Rust analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Homebrew](https://brew.sh) - package manager for MacOS/Linux
