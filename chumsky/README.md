## Chumsky
Is described as a parser library that uses parser combinator. So what are
parser combinators?

[Parser combinator](https://en.wikipedia.org/wiki/Parser_combinator) is
described as a higher order function that takes several parsers as input and
returns a new parser as output.
The way I understand this is that we define parsers for single item (token) that
we want to parse. We create parsers for each type of item we want to parse.

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

