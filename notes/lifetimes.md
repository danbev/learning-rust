## Lifetimes
Are actually named generic lifetime annotations and are declared in the same
way as genericts using `'` followed by a variable name.
For example `'a` is spoken as 'tick a':

Lifetimes are all about references and nothing else. There are used at compile
time by the borrowchecker and describe a relationship between references.

Rules:
* Each parameter that is a reference gets its own lifetime parameter

* If there is exactly `one` input lifetime parameter then that lifetime is
  assigned to all output lifetime parameters.

* If there are multiple input lifetime parameters but one of them is $self or
  &mut self then the lifetime self is assigned to all output lifetime parameters.

If any of these rules are "broken" the compile needs help from the programmer
to explicitly specify the generic lifetime annotations.

```console
error[E0597]: `y` does not live long enough
 --> snippets/src/lifetimes.rs:7:15
  |
7 |           x = &y;
  |               ^^ borrowed value does not live long enough
8 |       }
  |       - `y` dropped here while still borrowed
9 |       println!("x = {}", x);
  |                          - borrow later used here

```
The rust compiler can figure out lifetimes so that we don't have to specify them
but only if the following are true:
* The functions does not return a reference
* There is exactly one reference input parameter

When we pass a variable as opposed to a reference we are giving up ownership.
When passing a variable as a reference you are lending it to the function. You
can pass around as many immutable references as you like with out any issue.

One thing to notes is that lifetimes on function signatures can tell us what
a function can do with a passed in argument.

The following function cannot store the input reference in a place that would
outlive the function body (like static storage):
```rust
fn something<'a>(input: &'a i32) {
}
```

When a function takes a single ref as an argument and returns a single ref then
Rust assumes that those two refs have the same lifetime.

When we have a struct that has a generic lifetime annotation, for example:
```rust
struct SomeStruct<'a, T> {
    slice: &'a [T],
}

impl Iterator<'a, T> for SomeStruct<'a, T> {
  ...
}
```
Notice that we have to specify the lifetime annotation in the Iterator as well
as for SomeStruct. This is because the above Iterator is generic. If it was not
we could specify the implemention like this:
```rust
impl Iterator for SomeStruct<'static, i32> {
  ...
}
```
And that works because there is a generic lifetime annotation `'static` and
there is a concrete type `i32`.
