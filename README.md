### [Crust of Rust: Lifetime Annotations]()
Following along with discussion of how Rust Lifetimes work.

Creating a object that splits a string based on a delimiter and allows you to iterate over that original string, returning everything up to the next delimiter.

### Commit 1
`next()` -> 
1.  Find where the next delimiter appears in the remainder.  
2.  Chop off that part of the string and return that. 
3.  Set remainder to what remains after the delimiter.

### Commit 2 - Adding lifetimes
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

### Commit 3 - Adding lifetimes
*  Compile
    *  "Lifetime of reference outlives lifetime of borrowed content..." error

The problem here is that the thing returned has a different lifetime than the thing given.  We don't know that the haystack pointer lives as long as the StrSplit lifetime.  We need to define there relationship between the two.  Same thing for delimiter.

Compiler needs to ensure that as long as the StrSplit is around, the string we were given are still accessible through their pointers.

__How to express this relation?__
I can give you a StrSplit wiht a lifetime 'a if you give me string pointers that are also 'a:
```
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
```
before it was:
```
impl StrSplit<'_> {
    pub fn new(haystack: &str, delimiter: &str) -> Self {

```

Lifetimes can generally be thought about like types, with their relationships like subtypes. (probably won't need to express a relationship between lifetimes in this example project.)

__why `'a ` next to impl?__
same reason as this is invalid with generic types:
```
struct Foo<T> {}
```
The compiler would say that type T is undefined.  Putting them in front of the `impl` is how we define the type.  It makes the block a generic `impl` block over type T.  Similarly it makes the `impl` block generic over the lifetime 'a.

__why is the compiler okay with `self.remainder = "";`?__
Problem: `self.remainder` has type `&'a str` while `"";` has type `&'static str`.
A **static lifetime** is the lifetime that extends until the end of the program.  Any lifetime can be assigned to by a thing of a longer lifetime.

* Had to fix test as well.

