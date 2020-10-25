# ICPSR 36404 Analysis

Analysis of the [ICPSR 36404](https://www.icpsr.umich.edu/web/ICPSR/studies/36404) dataset
using descriptive machine learning.

## Usage

First, download the `delimited` version of the dataset. It is a tsv file, which is used as
input for the analysis program.

Then, [install the Rust stable toolchain](https://www.rust-lang.org/tools/install).

Compile this project with `cargo build --release`. No additional steps should be necessary
in order to compile.

The produced program provides the following usage:

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
        --admission-type <admission_type>    include only the given admission type [possible values: parole, new, other]
        --race <race>                        include only the given race [possible values: black, white, hispanic,
                                             other]
        --sex <sex>                          include only the given sex [possible values: male, female]

ARGS:
    <min_sup>    the minimum support ratio ([0, 1.0])
```

## Notebooks
fernanda <fernandaguimaraes28@gmail.com>

First, install the necessary `packages` to run the notebook:

`pip install scikit-learn`

`pip install pandas`

`pip install datetime`

`pip install numpy`

`pip install pysugbroup`

After that, it is necessary to put the data on the same folder, or alter the path in the notebook:

`data_path = "36404-0001-Data.tsv"`

That's it. Now just run the notebook with jupyter. You can also select the subgroup
`max_size` by altering the `depth` parameter in the `Subgroup Discovery` section.



## Additional crates

The following Rust crates were developed in order to support this work:
- [dci](https://docs.rs/dci/)
- [onehot](https://docs.rs/onehot/)
- [bitmatrix](https://docs.rs/bitmatrix/)

## Licence

This project is licenced under the [MIT Licence](http://opensource.org/licenses/MIT).
