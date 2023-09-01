# Playground Rust

## List of samples built for learning purposes

- [Palindrome](https://github.com/sheroz/palindrome)
- [Fibonacci](https://github.com/sheroz/fibonacci)
- [Shortest Path: Dijkstra](https://github.com/sheroz/shortest_path)
- [Tree and Parent-Child Relationship](https://github.com/sheroz/tree-samples-rs)
  - Generic Tree
  - Binary Tree
  - Binary Search Tree

## Useful links

- [The Book](https://doc.rust-lang.org/book)
- [The Rust Reference](https://doc.rust-lang.org/reference)
- [Rust Collections](https://doc.rust-lang.org/std/collections)
- [The Rust Programming Language](https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/README.html)
- [The Dark Arts of Unsafe Rust](https://doc.rust-lang.org/nightly/nomicon/)
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
- [Benchmarking](https://bheisler.github.io/criterion.rs/book/)

## Visual Studio Code

### DEBUG CONSOLE

#### Sample commands for debugging

```text
>help
>vo
>vo source_text
>x &source_text
>x 0x00555555597036
>x -c 256 0x00555555597036
>p source_text
>p &source_text
```

### Setting-up and using the [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) based code coverage

```text
$cargo +stable install cargo-llvm-cov --locked
$cargo llvm-cov
$cargo llvm-cov --html
$cargo llvm-cov --open 
```
