# region-code-formatter
A simple cli formatter for Chinese Administrative Region Code.


## Usage

```bash

region-code-formatter [FLAGS] [OPTIONS] <file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Use verbose output

OPTIONS:
    -d, --delimiter <delimiter>    Join region names with delimiter [default: /]
    -o, --output <output>          Write to <filename> [default: ./]

ARGS:
    <file>    The source file

```

## Example

```shell
$ region-code-formatter data/mixed.txt -o target/output.txt -d "" -v
```

## Official Data

[Ministry of Civil Affairs of the People's Republic of China](https://www.mca.gov.cn/n156/n2679)
