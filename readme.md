# Rust

## Day 1: 26/10/2026

### Hello World

Every Rust program starts execution from the `main()` function.

We use the `fn` keyword to declare a function and `println!` to print something to the console.

```rust
fn main() {
    println!("Hello, World!");
}
```

### Compilation

Rust is a compiled language. We use `rustc` to compile our Rust source code.

```powershell
rustc hello_world.rs
```

This generates two files:

- `hello_world.exe` – The executable file used to run the program.
- `hello_world.pdb` – Contains debugging information, useful while debugging the program.

To execute the program:

```powershell
.\hello_world.exe
```

**Output:**

```text
Hello, World!
```
