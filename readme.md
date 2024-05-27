# Getting started with Rust

<img src="learn-rust.jpeg" style="width: 25%" alt="ferris-learner">  

Resources, tools, and code samples for learning [Rust Programming Language](https://www.rust-lang.org/)

## Books

- Must Read: [The Book](https://doc.rust-lang.org/book)
- Must Read: [Rust for Rustaceans](https://rust-for-rustaceans.com/)
- [Programming Rust: Fast, Safe Systems Development](https://www.amazon.com/Programming-Rust-Fast-Systems-Development/dp/1492052590)
- [The Rust Programming Language](https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/README.html)
- [Easy Rust](https://dhghomon.github.io/easy_rust/)
  
### Courses & Workshops

- [Comprehensive Rust](https://github.com/google/comprehensive-rust) - This is a free Rust course developed by the Android team at Google. The course covers the full spectrum of Rust, from basic syntax to advanced topics like generics and error handling.
- [Learn to write Rust procedural macros](https://github.com/dtolnay/proc-macro-workshop) - Rust Latam conference, Montevideo Uruguay, March 2019.

### Other resources

- [This Week in Rust](https://this-week-in-rust.org/)
- [The Rust Reference](https://doc.rust-lang.org/reference)
- [Rust Collections](https://doc.rust-lang.org/std/collections)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust) - a curated list of Rust code and resources
- [Idiomatic Rust](https://github.com/mre/idiomatic-rust) - a collection of articles/talks/repos which teach concise, idiomatic Rust
- [The Rustonomicon](https://github.com/rust-lang/nomicon) - The Dark Arts of Advanced and Unsafe Rust Programming
- [The Rustonomicon - nightly](https://doc.rust-lang.org/nightly/nomicon/) - The Dark Arts of Unsafe Rust
- [Algorithms implemented in Rust](https://github.com/TheAlgorithms/Rust)
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
- [Advice for learning Rust](https://github.com/QuineDot/rust-learning) - [Learning Rust](https://quinedot.github.io/rust-learning/)
- [What Every Programmer Should Know About Memory](https://people.freebsd.org/~lstewart/articles/cpumemory.pdf) - explains the structure of memory subsystems in use on modern commodity hardware
- [Personal blog of Luca Palmieri](https://www.lpalmieri.com/) - has a good deal of useful stuff about Rust
- [Code of conduct](https://www.rust-lang.org/policies/code-of-conduct)

#### Concurrency

- [About Concurrency](https://assets.bitbashing.io/papers/concurrency-primer.pdf) - what every systems programmer should know about concurrency
- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)
- [Rust Is Hard, Or: The Misery of Mainstream Programming](https://hirrolot.github.io/posts/rust-is-hard-or-the-misery-of-mainstream-programming.html)
- [Async Rust Is A Bad Language](https://bitbashing.io/async-rust.html)

#### Benchmarking

- [Benchmarking: Criterion](https://bheisler.github.io/criterion.rs/book/)

## Tools

- [Visual Studio Code](https://code.visualstudio.com/docs/languages/rust)
  - Recommended plugins:
    - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
    - [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
    - [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
    - [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)
    - [SQLTools](https://marketplace.visualstudio.com/items?itemName=mtxr.sqltools)
    - [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client)
    - [Docker](https://marketplace.visualstudio.com/items?itemName=ms-azuretools.vscode-docker)
  - Keyboard shortcuts:
    - [Keyboard shortcuts for Linux](https://code.visualstudio.com/shortcuts/keyboard-shortcuts-linux.pdf)
    - [Keyboard shortcuts for macOS](https://code.visualstudio.com/shortcuts/keyboard-shortcuts-macos.pdf)
  - [Boost Your Coding Fu With Visual Studio Code and Vim](https://www.barbarianmeetscoding.com/blog/boost-your-coding-fu-with-vscode-and-vim)
- Take a look: [RustRover](https://www.jetbrains.com/rust/) - JetBrains IDE for Rust Developers
  - [VSCode Theme](https://plugins.jetbrains.com/plugin/19177-vscode-theme) - includes `VSCode Dark+` and `VSCode Dark Modern` themes

### Visual Studio Code - Debugging

- [Install debugging support](https://code.visualstudio.com/docs/languages/rust#_debugging)
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)

Also, try [`dbg!`](https://doc.rust-lang.org/std/macro.dbg.html)

```rust
dbg!(..);
```

Usage details and examples: [std::dbg](https://doc.rust-lang.org/std/macro.dbg.html)

#### Visual Studio Code - DEBUG CONSOLE (lldb)

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

### Clippy

- [Clippy](https://github.com/rust-lang/rust-clippy) - A collection of lints to catch common mistakes and improve your Rust code

## List of code samples built for learning purposes

- Web Service: REST API
  - [Getting started with Web Services in Rust (using axum, JWT, SQLx, PostgreSQL, and Redis)](https://github.com/sheroz/axum-web-api-kickstart)
- Cryptography
  - [Magma Symmetric Cipher](https://github.com/sheroz/magma)
  - [RSA Asymmetric Cipher](https://github.com/sheroz/rsa)
- [Tree and Parent-Child Relationship](https://github.com/sheroz/tree-samples-rs)
  - Generic Tree
  - Binary Tree
  - Binary Search Tree
- [Shortest Path: Dijkstra](https://github.com/sheroz/shortest_path)
- [Fibonacci](https://github.com/sheroz/fibonacci)
- [Palindrome](https://github.com/sheroz/palindrome)
  
