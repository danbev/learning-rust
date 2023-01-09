## clap with cargo custom command
This is an issue that I ran into then trying to run a cargo extension which
used clap.

For example, if we have a binary/application like
[cargo-non-filtered](cargo-example/src/non-filtered) and install it:
```console
$ cargo install --path .
```
And then try to run it using:
```console
$ cargo non-filtered --help
error: Found argument 'non-filtered' which wasn't expected, or isn't valid in this context

Usage: cargo-non-filtered --something <SOMETHING>

For more information try '--help'
```

And if we try [cargo-filtered](cargo-example/src/filtered) and run it:
```console
$ cargo filtered --help
args: Args { inner: ["/home/danielbevenius/.cargo/bin/cargo-filtered", "filtered", "--help"] }
filtered: ["/home/danielbevenius/.cargo/bin/cargo-filtered", "--help"]
Usage: cargo-filtered --something <SOMETHING>

Options:
  -s, --something <SOMETHING>  Some argument...
  -h, --help                   Print help information
  -V, --version                Print version information
```
