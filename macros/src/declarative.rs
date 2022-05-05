//! Declarative macros.

macro_rules! my_vec {
    ($elem:expr; $n:expr) => {
        ::std::vec::from_elem($elem, $n)
    };

    ( $($x:expr ),* ) => {
        <[_]>::into_vec(Box::new([ $( $x ),*]))
    };

    ( $($x:expr),+ ,) => {
        my_vec![ $( $x ),*]
    }
}

macro_rules! my_assert_eq {
    ($left: expr, $right: expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!(
                        "assertion failed: `(left == right)` \
                        (left: `{:?}`, right: `{:?}`)",
                        left_val, right_val
                    );
                }
            }
        }
    }};
}

macro_rules! my_matches {
    ($value:expr, $pattern:pat) => {{
        match $value {
            $pattern => true,
            _ => false,
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_my_vec() {
        let vec1 = my_vec![1, 2, 3, 4, 5];
        assert_eq!(vec1, &[1, 2, 3, 4, 5]);

        let vec2 = my_vec!["hola", "mundo"];
        assert_eq!(vec2, &["hola", "mundo"]);

        let vec3 = my_vec![1; 10];
        assert_eq!(vec3, &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_assert_eq() {
        let left = 1 + 2 * 3;
        let right = 2 * 3 + 1;
        my_assert_eq!(left, right);
    }

    #[test]
    #[should_panic]
    fn test_my_assert_eq_panic() {
        let left = 1 + 2 * 3;
        let right = 1 * 3 + 2;
        my_assert_eq!(left, right);
    }

    #[test]
    fn test_my_matches() {
        assert!(matches!(2 * 5, 10));
        assert!(my_matches!(2 * 5, 10));

        assert!(!matches!(2 * 3, 10));
        assert!(!matches!(2 * 3, 10));
    }
}
