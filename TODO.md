- how to print processor state for step debug?

```rust
struct State<WordSize> {
    program_counter: usize,
    regs: &[(&'static str, WordSize)],
    flags: &[(&'static str, bool)],
    data: &[u8],
}
```

- regression tests
