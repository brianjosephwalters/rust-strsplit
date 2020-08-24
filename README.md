### [Crust of Rust: Lifetime Annotations]()
Following along with discussion of how Rust Lifetimes work.

Creating a object that splits a string based on a delimiter and allows you to iterate over that original string, returning everything up to the next delimiter.

### Commit 1
`next()` -> 
1.  Find where the next delimiter appears in the remainder.  
2.  Chop off that part of the string and return that. 
3.  Set remainder to what remains after the delimiter.

### Commit 2
1.  Compile and add recommended lifetimes
    *  `'a` vs `'_` (anonymous lifetimes)
Rust needs to know how long it can hold onto pointers for. i.e,. we know that Item points to a location in remainder.  But it doesn't know how long it can use that pointer.  So `'a` tells us how long the references live for. 

__Do we always need to give a lifetime?__
There is only one lifetime in this context, Foo's.  So don't need to give the fully qualified lifetime.
struct Foo {}
impl Foo {
    fn get_ref(&self) -> &'_ str {}
}

**Anonymous lifetimes** (`'_`) are places where you tell the compiler it can guess which lifetime to use. 
    *  Similar to `_` for patern matching.

`'a` is like a generic variable.  The name itself doesn't matter.

__How does the compiler know a lifetime is wrong, but cannot infer it?__
```
fn multiply(x: (), y:i32) -> i32 {
    x*y
}
```
Compiler can tell you its wrong, but can't tell you what the right answer is.

__Why not ellide the lifetime?__
You can and should as much as possible, but sometimes you need it to eliminate lifetimes as well.  So the compiler can guess a different lifetime.
```
fn foo<'x, 'y>(x: &'x str, y: &'y str) -> &'x str{}
```
```
fn food(x: &str, y: &'_str) -> &'_ str {}
```
Here the idea is that in the declaration, the `'_` tells the compiler to ignore that one.  While in the return type, the compiler is being told by `'_` to guess.  Since y's lifetime was ignored, the only guess is x's.
    *  Syntactically confusing unless you understand what the lifetime variable means in a given context.

