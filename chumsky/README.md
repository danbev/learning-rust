## Chumsky
Is described as a parser library that uses parser combinator.
So what are parser combinators?

[Parser combinator](https://en.wikipedia.org/wiki/Parser_combinator) is
described as a higher order function that takes several parsers as input and
returns a new parser as output.

The way I understand this is that we define a parsers for single item (token)
that we want to parse. And we then create parsers for each type of item we want
to parse.

So we create a Parser for each type/item/token/thing:
```
  +------+     +------------+
  |Item1 |---->|Parser<Item1|
  +------+     +------------+
```
And then we can combine them into a larger parser which can handle all of them
items (again I think:)
```
  +--------------+      +-------------+         +-------------+
  | Parser<Item1>|----->|Parser<Item2>|-------->|Parser<Item3>|
  +--------------+      +-------------+         +-------------+
```
The combination of these parsers is a combinator.


## filter
This function is/can be imported using the prelude.
[filter](https://docs.rs/chumsky/latest/chumsky/primitive/fn.filter.html) has
the following definition:
```rust
pub fn filter<I, F: Fn(&I) -> bool, E>(f: F) -> Filter<F, E>
```
`I` stands for `Input` and it takes a function with a reference to a type of the
input, and the funtion returnes true if the item should be included, otherwise
it is filtered out:
```rust
    filter(|c: &char| c.is_ascii_digit()).map(|c| Item::Num(c.to_digit(10).unwrap()))
```

`Filter` is a struct and it has an implementation for `Parser`:
```rust
impl<I: Clone, F: Fn(&I) -> bool, E: Error<I>> Parser<I, I> for Filter<F, E> {
```

### Running
```console
$ cargo r --bin first first.foo
```
