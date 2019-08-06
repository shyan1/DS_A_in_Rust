# Hello Rust

The Rust language works with `Option` and `Result` types that let developers
model success or failure.

Only in rare cases when immediate termination is required does the language
provide a macro for panicking: `panic!()`.

`Option<T>` and `Result<T, E>` both encapsulate one (_`Option<T>`_) or two
(_`Result<T,E>`_) values that can be returned to communicate an error or whether
something was found or not.

```rust
fn find(needle: u16, haystack: Vec<u16>) -> Option<usize> {
    // ...
}

fn read_file(path: &str) -> Result<String, io::Error> {
    // ...
}
```

Handling those return values is often done with `match` or `if let` clauses in
order to handle the cases of success or failure:

```rust
match find(2, vec![1,2,3,4]) {
    Some(_) => println!("Found!"),
    None => println!("Not Found!"),
}

if let Some(result) = find(2, vec![1,2,3,4]) {
    println!("Found!")
}

match read_file("/tmp/not/a/file") {
    Ok(content) => println!(content),
    Err(_) => println!("oh no!"),
}
```

`Macros` are expanded in Rust code before compilation.

The rules of ownership:

- The owner of a value is a variable
- At any time, only a single owner is allowed
- The value is lost once the owner goes out of scope

---

- Every variable is owned by exactly one scope at any time
- Therefore, the developer is forced to pass ownership as required.

Rules of borrowing:

- Owners can have immutable or mutable references, but not both.
- There can be multiple immutable references, but only one mutable reference
- References cannot be invalid

- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

When s comes into scope, it is valid.
It remains valid until it goes out of scope.

`String literals` are immutable

`String` is allocated on the heap

```rust
let mut s = String::from("hello");
s.push_str(" world");
```

the memory is automatically returned once the variable that owns it goes out of scope.

```rust
{
    let s = String::from("hello");
    //...
} // s is no longer valid
```

When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`

Rust calls `drop` automatically at the closing curly bracket.

```rust
let s1 = String::from("hello");
let s2 = s1;        // s1 moved here
println!("{}", s1);             // compile error!!!

//

{
    let s1 = String::from("hello");
    let s2 = s1.clone();                // deep copy
    println!("{}, {}", s1, s2);         // Works OK
}
```

Passing a variable to a function will move or copy, just as assignment does.

References allow you to refer to some value without taking ownership of it.

```rust
fn calculate_length(str: &String) -> usize {
    s.len
}

fn change(str: &mut String) {
    str.push_str("!!!");
}

let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;        // Error: cannot borrow 's' as mutable more than once at a time

//

let mut s = String::from("hello");
{
    let r1 = &mut s;        // ok!
}
let r2 = &mut s;            // OK!
```

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

```rust
use std::thread;

fn threading() {
    let x = String::from("hello");

    let handle = thread::spawn(move || {
        println!("{}", x);
    });

    handle.join().unwrap();
}
```

The keyword `move` let a thread pass owner to another thread. It moves the memory content.

`mutex` (short for `mutual exclusion`), any time something is accessed within this locked mutext, it
is guaranteed to be single thread.

