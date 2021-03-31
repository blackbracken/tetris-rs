#[macro_use]
macro_rules! rect_vec {
    ($($x:expr),+ $(,)?) => (
        [ $($x),+ ].iter().map(|line| line.iter().map(|c| *c > 0).collect()).collect()
    )
}

pub(crate) mod game;
pub(crate) mod tetrimino;
pub(crate) mod board;