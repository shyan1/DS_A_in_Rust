#[allow(unused_macros)]
macro_rules! two {
    () => {
        1 + 1
    }
}

#[allow(unused_macros)]
macro_rules! calc {
    (two) => {
        1 + 1
    };
    (three) => {
        1 + 2
    }
}

#[allow(unused_macros)]
macro_rules! repeat_n_times {
    ($n: expr, $text: expr) => {
        (0..$n).map(|_| { format!("{}", $text)}).collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn two_works() {
        assert_eq!(2, two!());
    }

    #[test]
    fn calc_works() {
        assert_eq!(2, calc!(two));
        assert_eq!(3, calc!(three));

        // assert_eq!(4, calc!(four));      // no rules expected this token in macro call
    }

    #[test]
    fn releat_n_times_works() {
        assert_eq!(repeat_n_times!(2, "a"), vec!["a", "a"]);
    }
}