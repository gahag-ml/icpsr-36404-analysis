# ICPSR 36404 Analysis

Analysis of the [ICPSR 36404](https://www.icpsr.umich.edu/web/ICPSR/studies/36404) dataset
using descriptive machine learning.

## Usage
```
analyzer 0.1.0
gahag <gabriel.s.b@live.com>


USAGE:
    icpsr-36404-analysis [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    distribution    load the original dataset from stdin and display the data distribution
    help            Prints this message or the help of the given subcommand(s)
    load            load the serialized matrix from stdin and run the algorithm
    run             runs the entire pipeline
    save            load the original dataset from stdin and output the serialized matrix to stdout
```

```
icpsr-36404-analysis-run 
runs the entire pipeline

USAGE:
    icpsr-36404-analysis run [FLAGS] [OPTIONS] <min_sup>

FLAGS:
    -h, --help           Prints help information
        --recidivists    whether to include only recidivists
    -V, --version        Prints version information

OPTIONS:
    -s, --sex <sex>    include only the given sex [possible values: male, female]

ARGS:
    <min_sup>    the minimum support ratio ([0, 1.0])
```

## Additional crates

The following crates were developed in order to support this work:
- [dci](https://docs.rs/dci/)
- [onehot](https://docs.rs/onehot/)
- [bitmatrix](https://docs.rs/bitmatrix/)

## Licence

This project is licenced under the [MIT Licence](http://opensource.org/licenses/MIT).
