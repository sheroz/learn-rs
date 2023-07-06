# Learning Rust

## Useful links

[Rust Book](https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/README.html)

[Rust Collections](https://doc.rust-lang.org/std/collections/index.html)

[Benchmarking](https://bheisler.github.io/criterion.rs/book/)

## Visual Studio Code

### DEBUG CONSOLE

#### Sample useful commands for debugging (to be executed in DEBUG CONSOLE tab of VS Code)

    >help
    >vo
    >vo source_text
    >x &source_text
    >x 0x00555555597036
    >x -c 256 0x00555555597036
    >p source_text
    >p &source_text

### Test Coverage

    $cargo +stable install cargo-llvm-cov --locked
    $cargo llvm-cov
    $cargo llvm-cov --html
    $cargo llvm-cov --open 
