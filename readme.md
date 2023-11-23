# Learning Rust

Tools, resources and code samples for learning [Rust Programming Language](https://www.rust-lang.org/)

<img src="learn-rust.jpeg" width = 25%>  

## Tools

### Visual Studio Code

- [Rust in Visual Studio Code](https://code.visualstudio.com/docs/languages/rust)
- [Keyboard shortcuts for Linux](https://code.visualstudio.com/shortcuts/keyboard-shortcuts-linux.pdf)
- [Keyboard shortcuts for macOS](https://code.visualstudio.com/shortcuts/keyboard-shortcuts-macos.pdf)
- [Boost Your Coding Fu With Visual Studio Code and Vim](https://www.barbarianmeetscoding.com/blog/boost-your-coding-fu-with-vscode-and-vim)

### Debugging

- [Install debugging support](https://code.visualstudio.com/docs/languages/rust#_debugging)
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)

Also, try the simple and very useful `dbg!`:

```rust
dbg!(...)
```

Usage details and examples: [std::dbg](https://doc.rust-lang.org/std/macro.dbg.html)

#### DEBUG CONSOLE (lldb)

```text
>help
>v
>print a
>p &a
>x &a
>x 0x00555555597036
>x -c 256 0x00555555597036
>q
```

### Code Coverage

#### Setting-up and using the [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) based code coverage

```text
$cargo +stable install cargo-llvm-cov --locked
$cargo llvm-cov
$cargo llvm-cov --html
$cargo llvm-cov --open 
```

### Books

- [The Book](https://doc.rust-lang.org/book)
- [Programming Rust: Fast, Safe Systems Development](https://www.amazon.com/Programming-Rust-Fast-Systems-Development/dp/1492052590)
- [The Rust Programming Language](https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/README.html)
- [Easy Rust](https://dhghomon.github.io/easy_rust/)
  
### Other resources

#### Fundamentals

- [The Rust Reference](https://doc.rust-lang.org/reference)
- [Rust Collections](https://doc.rust-lang.org/std/collections)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Comprehensive Rust](https://github.com/google/comprehensive-rust) - the Rust course used by the Android team at Google
- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust) - a curated list of Rust code and resources
- [The Rustonomicon](https://github.com/rust-lang/nomicon) - The Dark Arts of Advanced and Unsafe Rust Programming
- [The Rustonomicon - nightly](https://doc.rust-lang.org/nightly/nomicon/) - The Dark Arts of Unsafe Rust
- [Algorithms implemented in Rust](https://github.com/TheAlgorithms/Rust)
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
- [Advice for learning Rust](https://github.com/QuineDot/rust-learning) - [Learning Rust](https://quinedot.github.io/rust-learning/)
- [What Every Programmer Should Know About Memory](https://people.freebsd.org/~lstewart/articles/cpumemory.pdf) - explains the structure of memory subsystems in use on modern commodity hardware
- [Personal blog of Luca Palmieri](https://www.lpalmieri.com/) - has many useful stuff about Rust

#### Concurrency

- [About Concurrency](https://assets.bitbashing.io/papers/concurrency-primer.pdf) - what every systems programmer should know about concurrency
- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)
- [Rust Is Hard, Or: The Misery of Mainstream Programming](https://hirrolot.github.io/posts/rust-is-hard-or-the-misery-of-mainstream-programming.html)
- [Async Rust Is A Bad Language](https://bitbashing.io/async-rust.html)

#### Benchmarking

- [Benchmarking: Criterion](https://bheisler.github.io/criterion.rs/book/)

### Recommended to explore

- [Tokio](https://tokio.rs/) - asynchronous runtime
- [axum](https://github.com/tokio-rs/axum) - web application framework
- [SQLx](https://github.com/launchbadge/sqlx) - async SQL toolkit

### List of samples for learning purposes

- [Palindrome](https://github.com/sheroz/palindrome)
- [Fibonacci](https://github.com/sheroz/fibonacci)
- [Shortest Path: Dijkstra](https://github.com/sheroz/shortest_path)
- [Tree and Parent-Child Relationship](https://github.com/sheroz/tree-samples-rs)
  - Generic Tree
  - Binary Tree
  - Binary Search Tree
- Cryptography
  - [Magma Symmetric Cipher](https://github.com/sheroz/magma)
  - [RSA Asymmetric Cipher](https://github.com/sheroz/rsa)
- Web
  - [Web Service](https://github.com/sheroz/axum-web)
  