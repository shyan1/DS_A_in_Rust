```rust
fn square_loop(mut x: u64) {
    loop {
        x = x * x;
    }
}
```

- Squaring any number small than 1 makes it smaller, so it approaches zero;
- Squaring 1 yields 1;
- Squaring a number larger than 1 makes it larger, so it approaches infinity;

---

```rust
fn square_add_loop(c: f64) {
    let mut x = 0.;
    loop {
        x = x * x + c;
    }
}
```

- If c is greater than 0.25, or less than -2.0, then x eventually becomes infinitely large;
- otherwise, it stays somewhere in the neighborhood of zero;

---

The `num` crate on [crates.io] provides a complex number type.

```rust
extern crate num;
use num::Complex;

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };       // complex zero
    loop {
        z = z * z + c;
    }
}
```
