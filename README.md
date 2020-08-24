### [Crust of Rust: Lifetime Annotations]()
Following along with discussion of how Rust Lifetimes work.

Creating a object that splits a string based on a delimiter and allows you to iterate over that original string, returning everything up to the next delimiter.

`next()` -> 
1.  Find where the next delimiter appears in the remainder.  
2.  Chop off that part of the string and return that. 
3.  Set remainder to what remains after the delimiter.