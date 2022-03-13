use derive_new::new;

/// テトリミノの形を表現する.
///
/// 形を表現するのに必要な最小のNを要求し,
/// NxNの正方形の行列を用いて真偽値でそのマスにミノが存在するかを表す.
#[derive(new, Clone, Copy)]
pub struct MinoMold<const N: usize = 3> {
    pub matrix: [[bool; N]; N],
}

impl<const N: usize> MinoMold<N> {
    pub fn square_n(matrix: [[u8; N]; N]) -> MinoMold<N> {
        let matrix = matrix.map(|ary| ary.map(|n| n != 0));

        MinoMold::new(matrix)
    }
}
