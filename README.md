<div align="center">
    <h1>memcell</h1>
    <img src="https://github.com/ImajinDevon/memcell/actions/workflows/rust.yml/badge.svg" alt="Rust build status badge">
</div>

## What is a MemoryCell?

A `MemoryCell` is a struct containing both a **current** and optional **previous** value.

#### Definition

```rust
#[derive(Debug, Clone)]
pub struct MemoryCell<T> {
    current: T,
    last_val: Option<T>,
}
```

## Features

- Full documentation
- Constant methods
- Lightweight (~10kb)
- Zero dependencies
- Pure Rust

## Example Usage

```rust
use memcell::MemoryCell;

fn main() {
    let mut cell = MemoryCell::new(5_u32);

    let new_value = 10;
    cell.update(new_value);

    assert_eq!(cell.current(), &10);
    assert_eq!(cell.last(), Some(&5));
}
```

[![Stargazers repo roster for @imajindevon/memcell](https://reporoster.com/stars/dark/imajindevon/memcell)](https://github.com/imajindevon/memcell/stargazers)
