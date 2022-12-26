# uuid-tool
Command line tool to generate uuid's (v4) in the terminal written in [Rust](https://www.rust-lang.org/).

# Usage

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

### Help
```shell
$ uuid-tool -h
Usage: uuid-tool [OPTIONS]

Options:
  -n, --number <NUMBER>  Number of uuids to generate [default: 1]
  -h, --help             Print help information
  -V, --version          Print version information
```
