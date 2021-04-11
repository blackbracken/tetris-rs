#[macro_export]
macro_rules! rect_vec {
    ($($x:expr),+ $(,)?) => (
        [ $($x),+ ].iter().map(|line| line.iter().map(|c| *c > 0).collect()).collect()
    )
}
