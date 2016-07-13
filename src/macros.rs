macro_rules! assert_fp { 
    ($a:expr, $b:expr) => (assert!(($a - $b).abs() < 0.001));
    ($a:expr, $b:expr, $c:expr) => (assert!(($a - $b).abs() < $c));
}
