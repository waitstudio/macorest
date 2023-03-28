#[macro_export]
macro_rules! hash_map {
    () => (std::collections::HashMap::new());
    ( $($x:expr, $y:expr;)+ ) => {{
        let mut m = hash_map!();
        $(m.insert($x, $y);)+
        m
    }}
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_map() {
        let m: HashMap<i32, i32> = hash_map!();
        assert_eq!(m.len(), 0);

        let m = hash_map!{
            1, 2;
            3, 4;
            5, 6;
        };
        assert_eq!(m.get(&1), Some(&2));
        assert_eq!(m.get(&3), Some(&4));
        assert_eq!(m.get(&5), Some(&6));
    }
}