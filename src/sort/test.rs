#[macro_export]
macro_rules! test {
    ($algo:expr, $is_less:expr) => {
        algo_test_cases! {
            $algo,
            $is_less,
            empty: (&mut [], &[0i32; 0]),
            single: (&mut [0], &[0]),
            sorted: (&mut [0, 1, 2], &[0, 1, 2]),
            reverse_sorted: (&mut [2, 1, 0], &[0, 1, 2]),
            all_duplicate: (&mut [0, 0, 0], &[0, 0, 0]),
            some_duplicate: (&mut [0, 1, 0], &[0, 0, 1]),
            some_duplicate_sorted: (&mut [0, 0, 1], &[0, 0, 1]),
            some_duplicate_reverse_sorted: (&mut [1, 0, 0], &[0, 0, 1]),
            alternating_duplicate: (&mut [1, 0, 1, 0, 1], &[0, 0, 1, 1, 1]),
            alternating_duplicate_2: (&mut [0, 1, 0, 1, 0], &[0, 0, 0, 1, 1]),
        }
    };
}

macro_rules! algo_test_cases {
        ($algo:expr, $is_less:expr, $($test_name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $test_name() {
                let (input, expected) = $value;
                $algo(input, $is_less);
                assert_eq!(expected, input);
            }
        )*
        }
    }
