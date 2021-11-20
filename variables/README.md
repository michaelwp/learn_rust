### variables

by default variable in rust is *immutable*, so to make it *mutable*, you have to add `mut` along with the variable.  
example : 

#### immutable variable
```rust
fn main() {
    let x = 45;
    println!("The value of x is {}", x);

    x = 60;
    println!("The value of x is {}", x);
}
```

will return :
```
cannot assign twice to immutable variable `x`
 --> src/main.rs:5:5
  |
2 |     let x = 45;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
...
5 |     x = 60;
  |     ^^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` due to previous error
```

---

#### mutable variable
```rust
fn main() {
    let mut x = 45;
    println!("The value of x is {}", x);

    x = 60;
    println!("The value of x is {}", x);
}
```
will return :
```
The value of x is 45
The value of x is 60
```