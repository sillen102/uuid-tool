# uuid-tool
Command line tool to generate uuid's (v4) in the terminal written in [Rust](https://www.rust-lang.org/).

## Installation

### Downloading binary

- Download the binary for your OS and architecture from the [release](https://github.com/sillen102/uuid-tool/releases/tag/v1.0.0)
- Place the binary in your PATH and rename it to `uuid-tool`

### Build from source
A pre-requisite is to have [Rust](https://www.rust-lang.org/) installed.

#### Mac OS
```shell
git clone https://github.com/sillen102/uuid-tool.git
```
```shell
cd uuid-tool
```
```shell
cargo build --release
```
```shell
sudo cp target/release/uuid-tool /usr/local/bin/uuid-tool
```

#### Linux
```shell
git clone https://github.com/sillen102/uuid-tool.git
```
```shell
cd uuid-tool
```
```shell
cargo build --release
```
```shell
sudo cp target/release/uuid-tool /bin/uuid-tool
```

## Usage

### Without specifying number of uuid's to generate
```shell
$ uuid-tool
4f7b9988-5837-4cf9-ae3f-d3ecdea24b89
```

### Specifying number of uuid's to generate
```shell
$ uuid-tool -n 5
39a5b5ee-2f82-4ecd-82d6-e2bea5742bdd
93d36dd1-50a0-46ab-9b6b-7237fff7934e
5f3aab10-4a14-4807-9913-c9a15edc1b63
a8a7a8a5-1c80-49fa-be94-d29bae8fe412
31067641-dd5c-41ea-b183-51fac01efc89
```

### Copy generated uuid's to clipboard
```shell
$ uuid-tool -c
c2b47001-1181-44a6-87d6-ad82be8718e2
Copied to clipboard
```

### Use custom separator
```shell
$ uuid-tool -s ' - ' -n 2
eab09f14-552a-4afd-b237-c73414f03d30 - 879c84eb-e057-4570-ba6f-1122c362b5c2
```

### Help
```shell
$ uuid-tool -h
Usage: uuid-tool [OPTIONS]

Options:
  -n, --number <NUMBER>        Number of uuids to generate [default: 1]
  -c, --copy-to-clipboard      Copy generated uuids to clipboard
  -s, --separator <SEPARATOR>  Separator between uuids [default: new line]
  -h, --help                   Print help information
  -V, --version                Print version information
```

## Cross compilation

The following commands work on an M1 Macbook Pro using the contents of this repository with cross installed.

### Prerequisites
- Rust
- Cross
- Docker

Install cross:
```shell
$ cargo install -f cross
```

### Windows (x86_64)
```shell
$ cross build --target x86_64-pc-windows-gnu --release 
```

### Linux (arm-v7)
```shell
$ cross build --target armv7-unknown-linux-gnueabihf --release
```

### Linux (arm64)
```shell
$ cross build --target aarch64-unknown-linux-gnu --release
```

### Linux (x86_64)
```shell
$ cross build --target x86_64-unknown-linux-gnu --release
```

### Mac OS (x86_64)
```shell
$ cross build --target x86_64-apple-darwin --release
```
