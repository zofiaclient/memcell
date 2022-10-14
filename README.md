# memcell

### Build & test status
![Build and test status badge](https://github.com/ImajinDevon/memcell/actions/workflows/rust.yml/badge.svg)

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
- Lightweight
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
    assert_eq!(cell.last_val(), &Some(5));
}
```