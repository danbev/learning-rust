## Rust Macro notes
When a source file is compiled it will first be converted into a sequence of
tokens like:

* identifiers
  something, self, x, y

* literals
  18, "something", 100_000
  
* keywords
  fn, match, macro, self, _

* symbols
  [, ::, ?, @


After this comes the parsing stage where the stream of tokens from above are
parsed into an abstract syntax tree (AST).
It is after this stage macros are processed.

### macro_rules! (Macros By Example MBE)

### Abstract Syntax Tree
The AST mirrors the structure of a Rust program in memory. There is a thing
called Span that is used to link parts back to their source code counter parts.

Lets take a look at some AST:
```console
$ make ast-tree
```
The macro in [simple_macro.rs](../src/simple_macro.rs) looks like this:
```rust
#[allow(unused_macros)]
macro_rules! doit {
    () => {}
}
```
So lets take a look at the AST-Tree for this:
```
Crate {
  attrs: [],
  items: [],
  span: src/simple_macro.rs:1:1: 8:2 (#0),
}
```
So the AST is created for the whole crate and the crate can have attributes
that are for the whole crate. For example if we added `#![allow(dead_code)]`
that would be part of the attributes list.

Now, I'm going to go through a section here which contains a TokenStream just
to get familiar with it. I'll skip the attributes for now and focus on an 
Item. Now like we could see above, items is a list of [
Item](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_ast/ast/struct.Item.html)'s
```rust
pub struct Item<K = ItemKind> {
    pub attrs: Vec<Attribute>,
    pub id: NodeId,
    pub span: Span,
    pub vis: Visibility,
    pub ident: Ident,
    pub kind: K,
    pub tokens: Option<LazyTokenStream>,
}
```

```
items: [                                                                    
        Item {                                                                  
            attrs: [...]

            id: NodeId(4294967040),                                             
            span: src/simple_macro.rs:4:1: 6:2 (#0),                            
            vis: Visibility {                                                   
                kind: Inherited,                                                
                span: src/simple_macro.rs:4:1: 4:1 (#0),                        
                tokens: None,                                                   
            },                                                                  
            ident: doit#0,                                                      
            kind: MacroDef(
                 MacroDef {                                                      
                    body: Delimited(                                            
                        DelimSpan {                                             
                            open: src/simple_macro.rs:4:19: 4:20 (#0),          
                            close: src/simple_macro.rs:6:1: 6:2 (#0),           
                        },                                                      
                        Brace,                                                  
                        TokenStream(
```
Notice that `kind` is `MacroDef` above which is our macro but there are other
kinds, [ItemKind](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_ast/ast/enum.ItemKind.html)
like `Const`, `Enum`, `ExternCrate`, `fn`, `ForeignMod`, `GlobalAsm`, `Impl`,
`MacCall` (macro invocation), `Mod`, `Static`, `Struct`, `Trait`, `TraitAlias`,
`TyAlias`, `Union`, and `Use`.

MacroDef is defined like this:
```rust
pub enum ItemKind {
  ...
  MacroDef(MacroDef),
```
So the enum element contains a [MacroDef](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_ast/ast/struct.MacroDef.html)
which is a struct:
```rust
pub struct MacroDef {
    pub body: P<MacArgs>,
    pub macro_rules: bool,
}

pub enum MacArgs {
    Empty,
    Delimited(DelimSpan, MacDelimiter, TokenStream),
    Eq(Span, MacArgsEq),
}

pub struct DelimSpan {
    pub open: Span,
    pub close: Span,
}

pub enum MacDelimiter {
    Parenthesis,
    Bracket,
    Brace,
}
```
[Span](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_span/span_encoding/struct.Span.html)
shows up in a lot of places and like mentioned earlier this is a connection back
to the source code line/column that this AST element originated from.
```rust
pub struct Span {
    base_or_index: u32,
    len_or_tag: u16,
    ctxt_or_zero: u16,
}
```
Note that Span has its own crate named `rustc_span`. Seeing this struct the
first time was a little confusing for me as I lacked some details about macros
and the context they have which is releated to name spaces. This is, at least
this is my current understanding, the reason for the field `ctxt_or_zero` which
would be a id for the namepace (context) for the macro. This has to do with
something called higiene which I'll create a separate section for. 

If we look at the `DelimSpan` in our example output above we can see:
```
                        DelimSpan {                                             
                            open: src/simple_macro.rs:4:19: 4:20 (#0),          
                            close: src/simple_macro.rs:6:1: 6:2 (#0),           
```
`open` corresponds to 
```
4                  19
↓                   ↓
4 macro_rules! doit {
5    () => {}
6 }
↑ ↑
6 1
```
Gettting back to our example we next have:
```
            kind: MacroDef(
                 MacroDef {
                    body: Delimited(
                        DelimSpan {
                            open: src/simple_macro.rs:4:19: 4:20 (#0),
                            close: src/simple_macro.rs:6:1: 6:2 (#0),
                        },
                        Brace,
                        TokenStream(
```
`Brace` is part of the enum MacDelimiter. So if we were to change our example
to use [], would this change to Brackets?  
Yes, that is the case, we are also required in this case to add a semicolon
after the bracket to allow this to compile.

After this we have the [TokenStream](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_ast/tokenstream/struct.TokenStream.html):
```rust
pub type TreeAndSpacing = (TokenTree, Spacing);

pub struct TokenStream(pub(crate) Lrc<Vec<TreeAndSpacing>>);

pub enum Spacing {
    Alone,
    Joint,
}
```
So a TokenStream instance will have a single member, 0, which is of type
`Lrc<Vec<TreeAndSpacing>` and TreeAndSpacing is a tuble of TokenTree and
Spacing. So we have a referenced counted instance to a vector of these tuples.
[Lrc](https://doc.rust-lang.org/stable/nightly-rustc/rustc_data_structures/sync/struct.Lrc.html).

[TokenTree](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_ast/tokenstream/enum.TokenTree.html):
```rust
pub enum TokenTree {
    Token(Token),
    Delimited(DelimSpan, Delimiter, TokenStream),
}

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub enum TokenKind {
    Eq,
    Lt,
    Le,
    EqEq,
    Ne,
    Ge,
    Gt,
    AndAnd,
    OrOr,
    Not,
    Tilde,
    BinOp(BinOpToken),
    BinOpEq(BinOpToken),
    At,
    Dot,
    DotDot,
    DotDotDot,
    DotDotEq,
    Comma,
    Semi,
    Colon,
    ModSep,
    RArrow,
    LArrow,
    FatArrow,
    Pound,
    Dollar,
    Question,
    SingleQuote,
    OpenDelim(Delimiter),
    CloseDelim(Delimiter),
    Literal(Lit),
    Ident(Symbol, bool),
    Lifetime(Symbol),
    Interpolated(Lrc<Nonterminal>),
    DocComment(CommentKind, AttrStyle, Symbol),
    Eof,
}
```

### TokenTree
This is somehing inbetween the tokens and the AST and consists of leaves and
non-leaves like (...), [...], and {...}.
So an expression would be tokenized it separate parts:
```
 x + y
```
`x` would be a leaf, `+` would be a leaf`, and `y` would be a leaf.
If we add one of the grouping construct that will introduce a tree:
```
 x + y + (w * v)
```
Here `(w *  v)` will become a tree. So why is this important. Well when we
use a macro:
```rust
some_macro! args
```
What we are passing in as args is a token tree, which can be `(...)`, `[...]`,
or `{...}`. This might be the reason that when invoking a macro_rules like this
we can chose which type of delimiter to be used, (), [], or {}.


### High-Level Intermediate Representation
Is what is used most in the rust compiler. It is created by a process called
lowering where the AST is converted into HIR. Some structures that are not
relevent for type analysis can be removed when lowering to HIR.

```console
$ rustc -Z unpretty=hir-tree - <<HERE
> fn main() {
> }
> HERE
```

### Hygiene
This is about the macro not causing any conflicts with it surrounding. Like
variable names that exist in the macro and might also be present in the scope
where the macro is being called from.


### Matchers
In the macro itself we can write anything really.
```
macro_rules! doit [
    (bajja, 123! ::something) => { let _x = 10;}
];

fn main() {
    doit!(bajja, 123! ::something);
}
```
And the only things that will be in our main function after this macro has been
expanded will be:
```console
fn main() { let _x = 10; }
```
Now, like with regular expressions it is possible to capture the input arguments
like bajja, 123! and ::something above. When we do this we have to tell Rust
what kind/type the capture is. The syntax for this is `$identifier:kind`.

```rust
macro_rules! doit [
    ($local_bajja:ident, 123! ::something) => { let $local_bajja = 10;}
];

fn main() {
    doit!(bajja, 123! ::something);
}
```


### Procedural Macros
Allow for code to be run at compile time that operates on Rust syntax and
outputs Rust syntax, so it operates on the AST and outputs/transforms that AST.


A crate needs to be marked as `proc-macro` using one of the following ways:
```
--crate-type=proc-macro, #[crate_type = "proc-macro"]
```
Or if using Cargo:
```
[lib]
proc-macro = true
```

There are three types of proc macros:
* Derive which enable the type of macros like #[derive(MacroName)], like
  #[derive(Debug)].

* Function like macros like macro_name!, for example env!("PATH").

* Attribute macros which allow annotations on functions which can perform
 transformations on the code at compile time.


