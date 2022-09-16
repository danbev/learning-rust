## lang_item
[lang_item](https://doc.rust-lang.org/beta/unstable-book/language-features/lang-items.html)
is decribes as a pluggable feature that the rust compiler has. 

A full list of all `lang_items`'s can be found in
[rustc_hir::lang_items::LangItem](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/lang_items/enum.LangItem.html).

This document attempts to show how one such lang_item works, namely
`unsafe_cell`.

### UnsafeCell
UnsafeCell can be found in `rust/library/core/src/cell.rs` and is declared like
this:
```rust
#[lang = "unsafe_cell"]
#[repr(transparent)]
pub struct UnsafeCell<T: ?Sized> {
    value: T,
}
```
So looking at the struct is is just one field, `value` of type T.

Also note the usage of a lang item, `unsafe_cell`.
If we look in `rust/compiler/rustc_hir/src/lang_items.rs` we find:
```rust
language_item_table! { 
//  Variant name,        Name,                     Method name,                Target                  Generic requirements;
UnsafeCell,              sym::unsafe_cell,         unsafe_cell_type,           Target::Struct,         GenericRequirement::None;
}
```
Ok, so what does the `language_item_table` macro do?

Lets take a closer look:
```rust
macro_rules! language_item_table {                                              
(                                                                           
     $( $(#[$attr:meta])* $variant:ident $($group:expr)?, $module:ident :: $name:ident, $method:ident, $target:expr, $generics:expr; )*
) => {              
```
One things to note here which got me the first time, is that (group:expr) is
optional and not used for UnsafeCell, but is used for other lang_items like 
`Add(Op)`.

The following is an approximation of what the macro will be expanded into: 
```rust
pub enum LangItem {
  ...
  UnsafeCell,
  ...
}

impl LangItem { 

  pub fn name(self) -> Symbol {                                       
     match self {                                                    
        ...
        LangItem::UnsafeCell => unsafe_cell_type,
        ...
     }                                                               
  }    

  pub fn group(self) -> Option<LangItemGroup> {                       
     use LangItemGroup::*;                                           
         match self {                                                    
	     LangItem::UnsafeCell => expand_group!(sym:unsafe_cell_type,
             $( LangItem::$variant => expand_group!($($group)*), )*         
         }                                                               
  }    

pub struct LanguageItems {
  ...
 $(                                                                  
    #[doc = concat!("Returns the [`DefId`] of the `", stringify!($name), "` lang item if it is defined.")]
    pub fn $method(&self) -> Option<DefId> {                        
        self.items[LangItem::$variant as usize]                     
   }                                                               
 )*   
}
```
So LanguageItems will have a function named [unsafe_cell_type](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/lang_items/struct.LanguageItems.html#method.unsafe_cell_type)
Lets find out where this function is used:
```console
$ grep -Rn unsafe_cell_type compiler/* --exclude=rusty-tags.vi
compiler/rustc_hir/src/lang_items.rs:219:    UnsafeCell,              sym::unsafe_cell,         unsafe_cell_type,           Target::Struct,         GenericRequirement::None;
compiler/rustc_middle/src/ty/adt.rs:252:        if Some(did) == tcx.lang_items().unsafe_cell_type() {
compiler/rustc_typeck/src/variance/terms.rs:111:        (lang_items.unsafe_cell_type(), vec![ty::Invariant])
```

One usage is in `compiler/rustc_middle/src/ty/adt.rs`
```rust
pub struct AdtFlags: u32 {
     ...
     /// Indicates whether the type is `UnsafeCell`.
     const IS_UNSAFE_CELL              = 1 << 9;
}

impl AdtDefData {
    pub(super) fn new(
        tcx: TyCtxt<'_>,
        did: DefId,
        kind: AdtKind,
        variants: IndexVec<VariantIdx, VariantDef>,
        repr: ReprOptions,
    ) -> Self {
        debug!("AdtDef::new({:?}, {:?}, {:?}, {:?})", did, kind, variants, repr);
        let mut flags = AdtFlags::NO_ADT_FLAGS;
    }
    ...
    if Some(did) == tcx.lang_items().unsafe_cell_type() {
        flags |= AdtFlags::IS_UNSAFE_CELL;
    }
    ...
}

    /// Returns `true` if this is UnsafeCell<T>.
    #[inline]
    pub fn is_unsafe_cell(self) -> bool {
        self.flags().contains(AdtFlags::IS_UNSAFE_CELL)
    }
```
So lets follow this and see where `is_unsafe_cell` is called.

__wip__

Lets try enableing logging and see if we can identify the log statement above
```console
$ RUSTC_LOG=rustc_middle::ty=debug make -B out/unsafecell 2> output
```
