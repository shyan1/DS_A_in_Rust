#![feature(test)]
extern crate test;

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::add;
    use test::Bencher;

    use std::thread;
    use std::time::Duration;
    #[test]
    fn this_works() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn this_does_not_work() {
        assert_eq!(add(std::i32::MAX, std::i32::MAX), 0);
    }

    #[bench]
    fn how_fast(b: &mut Bencher) {
        b.iter(|| add(42, 42));
        thread::sleep(Duration::from_millis(5000));
    }
}
