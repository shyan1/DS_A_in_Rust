use std::mem;

#[allow(dead_code)]
struct MyStruct {
    a: u8,
    b: u8,
    c: u8,
}

fn main() {
    // pub const fn size_of<T>() -> usize
    // for any type T and length n, [T; n] has a size of n * size_of::<T>().
    assert_eq!(mem::size_of::<MyStruct>(), 3 * mem::size_of::<u8>());
    assert_eq!(mem::size_of::<[MyStruct; 2]>(), mem::size_of::<u8>());
    println!("{}", mem::size_of::<MyStruct>());
}