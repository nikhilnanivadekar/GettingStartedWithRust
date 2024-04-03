# GettingStartedWithRust

# Abstract

Rust focuses on speed, safety, semantics, and productivity. Rust can be considered a multi-paradigm language. As a Java developer, Nikhil had to learn the same concepts and use them differently. In the 100% hands on talk, Nikhil will explain the basics of Rust, some different yet cool features the sets Rust apart from other programming languages. The audience will learn some of the basic concepts, advantages, disadvantages of Rust and decide if they would like to "Get started with Rust".

# Notes
-  `fn main()` is the entry point of a program.
- Parameters go in parentheses
- `let` is used to define a variable: What about data types?
- `mut` is used to make a variable mutable
  - Immutable by default
- Standard Data Structures: `List`, `Set`, `Map`
- Code Arrangement in Modules
- Inheritance: Struts
- Enums
- Function Access
- Variable Scopes: Reference and value
- Testing

# Naming Conventions
Available here: https://rust-lang.github.io/api-guidelines/naming.html

# Ownership
- System of ownership is used for memory management
- Compiler checks for a set of rules: 
  - If any of the rules are violated, it is a compile time error
  - Hence, at runtime ownership does not slow down the execution time
- Ownership helps to develop code that is safe and efficient
- Ownership is one of the concepts that makes Rust unique
- Ref: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

## Ownership Rules
- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

# Difference between `str` (String Slice) and `String`
- Ref: https://dev.to/dsysd_dev/string-vs-str-in-rust-understanding-the-fundamental-differences-for-efficient-programming-4og8
## Ownership
- `str` is a borrowed reference to a portion of an existing string or string literal, and it does not own the data.
- `String` owns the data it represents and is responsible for its memory allocation and deallocation.
## Allocation
- `str` does not require heap allocation
- `String` is heap allocated

