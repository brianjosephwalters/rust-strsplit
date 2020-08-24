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
```rust 
struct Foo {}
impl Foo {
    fn get_ref(&self) -> &'_ str {}
}
```
**Anonymous lifetimes** (`'_`) are places where you tell the compiler it can guess which lifetime to use. 
    *  Similar to `_` for patern matching.

`'a` is like a generic variable.  The name itself doesn't matter.

__How does the compiler know a lifetime is wrong, but cannot infer it?__
```rust
fn multiply(x: (), y:i32) -> i32 {
    x*y
}
```
Compiler can tell you its wrong, but can't tell you what the right answer is.

__Why not ellide the lifetime?__
You can and should as much as possible, but sometimes you need it to eliminate lifetimes as well.  So the compiler can guess a different lifetime.
```rust
fn foo<'x, 'y>(x: &'x str, y: &'y str) -> &'x str{}
```
```rust
fn food(x: &str, y: &'_str) -> &'_ str {}
```
Here the idea is that in the declaration, the `'_` tells the compiler to ignore that one.  While in the return type, the compiler is being told by `'_` to guess.  Since y's lifetime was ignored, the only guess is x's.
    *  Syntactically confusing unless you understand what the lifetime variable means in a given context.

### Commit 3 - Adding lifetimes
*  Compile
    *  "Lifetime of reference outlives lifetime of borrowed content..." error

The problem here is that the thing returned has a different lifetime than the thing given.  We don't know that the haystack pointer lives as long as the StrSplit lifetime.  We need to define there relationship between the two.  Same thing for delimiter.

Compiler needs to ensure that as long as the StrSplit is around, the string we were given are still accessible through their pointers.

Note: We are writing code that generic over lifetimes.  Otherwise, lifetimes *are* infered.  Similar to generic types.  We want to write collections that are generic over types.  Otherwise (in Java) we could use Object and let the compiler - and us - infer types.

__How to express this relation?__
I can give you a StrSplit wiht a lifetime 'a if you give me string pointers that are also 'a:
```rust
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
```
before it was:
```rust
impl StrSplit<'_> {
    pub fn new(haystack: &str, delimiter: &str) -> Self {

```

Lifetimes can generally be thought about like types, with their relationships like subtypes. (probably won't need to express a relationship between lifetimes in this example project.)

__why `'a ` next to impl?__
same reason as this is invalid with generic types:
```rust
struct Foo<T> {}
```
The compiler would say that type T is undefined.  Putting them in front of the `impl` is how we define the type.  It makes the block a generic `impl` block over type T.  Similarly it makes the `impl` block generic over the lifetime 'a.

__why is the compiler okay with `self.remainder = "";`?__
Problem: `self.remainder` has type `&'a str` while `"";` has type `&'static str`.
A **static lifetime** is the lifetime that extends until the end of the program.  Any lifetime can be assigned to by a thing of a longer lifetime.

If you have a value assigned to a variable, the lifetime of the value is until it is moved.  If it is never moved, it has a static lifetime.  If you store a value on the stack of a function, the lifetime of the value is the lifetime of that function.  When function returns, the lifetime ends.

Constant strings (written directly) is compiled into the binary.  When the program is launched, it is in some read only memory that will never move.  So a pointer to it is static.  

* Had to fix test as well.

### Commit 4 - Tail delimiter
Introduced a new test to handle trailing delimiters.  trailing delimiters should return the empty string.
Need to distinguish between:
    *  Remainder is empty
    *  Remainder is an empty element we haven't yielded yet.
1.  Option the remainder.  This allows us to check for either Some or None
    *  If there is Some(remainder) left, and for that remainder there is Some(delimiter) we can find, then do our work.
    *  If there is Some(remainder) left, but there are no more delimiters, then `.take()` what's left and return None.
    *  If there is no remainder left, then return None
2.  Adjust the use of the remainder (for the first case) since it now lives in an Option.
    *  We get the remainder out of the Option as a `ref mut remainder`.
    *  We get a reference to the next delimiter: `&remainder[..next_delim]`
    *  But then we need to update the remainder itself by chopping off everything before the delimiter.  Or, really, since remainder is a pointer to the start of the string, we just update that point with a new address: `*remainder = &remainder[(next_delim + self.delimiter.len())..];`  Which reads: "remainder, as a pointer, needs to now point to the memory location pointed at by `&remainder[...]`."  "I want to assign the RHS into where remained is pointing."


__Is heap allocation always a static lifetime?__
No, heap allocation lives until it is dropped.  So it has *some* lifetime.  If it's never dropped, it would be static.  Generally, box_leak returns a static reference.

If you dump the binary you could spot the static allocations. (except "" which get optimized out.)
    * Unix: `strings` prints all strings in a binary.

__What is the ref keyword?__
If I do `if let Some(remainder) = self.remainder {...}`, it *moves* out of self.remainder.  It assumes that I own `self.remainder` and I want to move the body.  But that's not what we want to do.  I want to get a mutable *reference* to the value inside of self.remainder, if it is Some().  
    *  the type of `self.remainder` is an `Option<&'a str>`.  But we want the type of remainder (the local one) to be `&mut &'a str` - "a mutable reference to the 'a str".  Without `ref mut`, we would get `&'a str`.  The latter wouldn't help because I need to reassign that value to move it to be beyond the next delimiter.  I don't want to *take* the value, I want to modify the existing one.

So:  `Some(remainder)` would *take* the value, `Some(mut ref remainder)` allows us to modify the existing value.

*  `ref x` means that I'm matching into a reference.  I want a reference to the thing I'm matching, rather than the thing I'm matching itself.
*  `ref mut x` means that I want to get a mutable reference to the thing I'm matching, rather than the thing I'm matching itself.

__Why can't I use `Some(&mut remainder)`?__
`Some(&mut x)` sort of does the opposite of what we wanted above.  "Take what the RHS is, and try to match it against the pattern in the Some().  `Some(&mut remainder)` would only match something that is a `Option<&mut T>`.  And then remainder would be `T`.  What we want is a mutable reference _after_ the pattern match.  I.e., Pattern match `T`, then give us a `&mut T`.

*  With new automagic things, we could also do `if let Some(remainder) = &mut self.delimiter {` but there is more magic going on.
*  `ref` = "take a reference to"

__What is the deref of remainder doing? `*remainder`__
The type of remainder is `&mut &'a str`, but RHS is `&'a str`.  We want to assign RHS "into where remainder is point".  Thus we dereference it.

__Are we selecting past the end of the string?__
`&remainder[(next_delim + self.delimiter.len())..]` may give us one past the end of the string.  This can be valid position to slice a string at and it gives the empty slice.

__What is take() doing?__
It is implemented on `Option<T>`.  `take()` takes a mutable reference to the option and gives you back an Option<T>.  If the Option is None, then it returns None.  If the Option is Some then it sets the Option to None and returns the Some that was in there.
*  `impl<T> Option<T> {fn take(&mut self) -> Option<T>}`
*  We only want to return "the remainder that doesn't have a delimiter" once.  That's what this will allow us to do.

### Commit 5 - Simplification
The `?` try operator also works on Options.

Every `let` statement is a pattern match.  I want to pattern match on what was inside the Some() of self.remainder in order to take a reference to what was in there:
```rust
if let Some(ref mut remainder) = self.remainder {
    ...
}
else {
    None
}
```
is equivalent to:
```rust
let ref mut remainder = self.remainder?;
```
You could also write the latter as:
```rust
let remainder = &mut self.remainder?;
```
They are sort of inverses of each other.  **The Two Above are not Quite Right.** Fixed below.

__If self is mutable, why isn't self.remainder not mutable by default?__
Mutable references are only one level deep.  A mut to self `fn next(&mut self)` means you can modify any fields of Self (remainder, delimiter).  But delimiter is an *immutable* pointer to some string.  You can make delimiter point somewhere else, but not change the thing that delimiter is pointing to.  For the latter case, delimiter itself would have to be a mutable reference.

__Why change `self.remainder?` to `self.remainder.as_mut()?` ?__
`let remainder = self.remainder?;`: 
    *  if self.remainder is None, then it returns None
    *  otherwise it returns the value inside the Some().  
Normally that would move the T that's inside the Some().  But because the T inside the Some() is "copy" (i.e., `&'a str`), we get **copy** semantics instead of **move** semantics.  Which means it copies the reference outside the option.  This means the remainder is no longer the same remainder as the one defined by the struct.

Therefore, when we modify it on the next line, we are actualy modifying our copy of that pointer, not the pointer stored inside self.

`as_mut()` is a function on Option<T> that takes a mutable reference to self and returns an option that contains a mutable reference to self.
    *  `impl<T> Option<T> { fn as_mut(&mut self) -> Option<&mut T> }`



