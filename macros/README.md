# Macros

`macros` make your code more efficient, less repetitive, and in general better without much complexity at all.

There are two types of macros: `procedural` and `declarative`.

- Procedural macros are more complex to write but (IMHO) easier to use - we have all used `#[derive(Debug)]` to make the `Debug` trait implementation magically appear.
- These are also sometimes referred to as “macros by example,” “macro_rules! macros,” or just plain “macros.”

```rust
macro_rules! two {
    () => { 1 + 1 }
}

assert_eq!(2, two!());
```

As any macros, it has three parts:

- a name ( `two` )
- a matcher ( `()` )
- a transcriber containing an expression( `{ 1 + 1 }` )

Each parameter has a name (`$name`) and a type (called `designator`) that matches on the parameters that are passed into the macro. There are several designators to match to (incl. reference): `ident`, `block`, `stmt`, `expr`, `pat`, `ty`, `lifetime`, `literal`, `path`, `meta`.

```rust
#[derive(PartialEq, Debug)]
struct Response(usize)

// Declare a function in macro
macro_rules! handler {
    ($i: ident, $body: block) => {
        fn $i () => Response $body
    }
}
```
