# Code samples and other resources for learning Rust

## List of my samples built for learning purposes

- [Palindrome](https://github.com/sheroz/palindrome)
- [Fibonacci](https://github.com/sheroz/fibonacci)
- [Shortest Path: Dijkstra](https://github.com/sheroz/shortest_path)
- [Tree and Parent-Child Relationship](https://github.com/sheroz/tree-samples-rs)
  - Generic Tree
  - Binary Tree
  - Binary Search Tree
- [Magma symmetric cipher](https://github.com/sheroz/magma)
- [RSA asymmetric cipher](https://github.com/sheroz/rsa)
  
## Useful

### Books

- [The Book](https://doc.rust-lang.org/book)
- [Programming Rust: Fast, Safe Systems Development](https://www.amazon.com/Programming-Rust-Fast-Systems-Development/dp/1492052590)
- [The Rust Programming Language](https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/README.html)
- [Easy Rust](https://dhghomon.github.io/easy_rust/)
  
### Other resources

- [The Rust Reference](https://doc.rust-lang.org/reference)
- [Rust Collections](https://doc.rust-lang.org/std/collections)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [The Dark Arts of Unsafe Rust](https://doc.rust-lang.org/nightly/nomicon/)
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
- [About Concurrency](https://assets.bitbashing.io/papers/concurrency-primer.pdf) - what every systems programmer should know about concurrency
- [Benchmarking: Criterion](https://bheisler.github.io/criterion.rs/book/)
- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust) - a curated list of Rust code and resources
- [Comprehensive Rust](https://github.com/google/comprehensive-rust) - the Rust course used by the Android team at Google
- [Algorithms implemented in Rust](https://github.com/TheAlgorithms/Rust)
- [Advice for learning Rust](https://github.com/QuineDot/rust-learning) - [Learning Rust](https://quinedot.github.io/rust-learning/)
   
## Visual Studio Code

- [Rust in Visual Studio Code](https://code.visualstudio.com/docs/languages/rust)

### Debugging

- [Install debugging support](https://code.visualstudio.com/docs/languages/rust#_debugging)
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)

Also simple and very useful:

```rust
dbg!(...)
```

Usage details and examples: [std::dbg](https://doc.rust-lang.org/std/macro.dbg.html)

#### DEBUG CONSOLE

```text
>help
>vo
>vo a
>x &a
>x 0x00555555597036
>x -c 256 0x00555555597036
>p a
>p &a
```

### Code Coverage

#### Setting-up and using the [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) based code coverage

```text
$cargo +stable install cargo-llvm-cov --locked
$cargo llvm-cov
$cargo llvm-cov --html
$cargo llvm-cov --open 
```
