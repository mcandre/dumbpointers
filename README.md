# dumbpointers: Pointless exercises with Rust (de)allocation

# EXAMPLE

```rust
let tc = Tc::new(1337, Duration::from_millis(1));
assert_eq!(*tc, 1337);

thread::sleep(Duration::from_millis(2));

assert_eq!(tc.examine(), Option::None);
```

# RUNTIME REQUIREMENTS

(None)

# CONTRIBUTING

For more details on developing dumbpointers itself, see [DEVELOPMENT.md](DEVELOPMENT.md).
