#[macro_export]
macro_rules! max {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => ($x.max(max!($($y),+)));
    ($x:expr;) => ($x.iter().max());
    ($x:expr; $s:expr, $e:expr) => ($x[$s..$e].iter().max());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_number() {
        assert_eq!(max!(1), 1);
        assert_eq!(max!(1, 3, 2), 3);
    }

    #[test]
    fn test_vec() {
        assert_eq!(max!(vec![1, 3, 2, 8, 5];), Some(&8));
        assert_eq!(max!(vec![1, 3, 2, 8, 5]; 0, 3), Some(&3));
    }
}