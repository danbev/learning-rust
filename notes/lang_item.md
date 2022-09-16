## lang_item
[lang_item](https://doc.rust-lang.org/beta/unstable-book/language-features/lang-items.html)
is decribes as a pluggable feature that the rust compiler has. This document
attempts to show how one such lang_item works, namely `unsafe_cell`.

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
optional and not used for UnsafeCell, but is used for example for `Add(Op)`.
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
