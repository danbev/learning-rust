## clap with cargo custom command
This is an issue that I ran into then trying to run a cargo extension which
used clap.

### Error
When doing this and trying to run the command using `cargo command`
I get an error similar to this:
```console
$ cargo command --help
error: Found argument 'command' which wasn't expected, or isn't valid in this context
```

### Reproducer
For example, if we have a binary/application like
[cargo-non-filtered](../cargo-example/src/non-filtered.rs) and install it:
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

### Possible solution
One solution to this that works is to filter the arguments before passing them
to clap. [cargo-filtered](../cargo-example/src/filtered.rs) contains an example
of doing this can can be run using:
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
Notice the arguments are printed before and after the filtering, and that there
is a second argument `filtered` in this case, and that this is getting filtered
out before clap is parsing the arguments.
