use derive_new::new;

/// テトリミノの形を表現する.
///
/// 形を表現するのに必要な最小のNを要求し,
/// NxNの正方形の行列を用いて真偽値でそのマスにミノが存在するかを表す.
#[derive(new)]
pub struct MinoMold<const N: usize> {
    pub matrix: [[bool; N]; N],
}

impl<const N: usize> MinoMold<N> {
    pub fn square2(matrix: [[u8; 2]; 2]) -> MinoMold<2> {
        MinoMold::square_n(matrix)
    }

    pub fn square3(matrix: [[u8; 3]; 3]) -> MinoMold<3> {
        MinoMold::square_n(matrix)
    }

    pub fn square4(matrix: [[u8; 4]; 4]) -> MinoMold<4> {
        MinoMold::square_n(matrix)
    }

    fn square_n(matrix: [[u8; N]; N]) -> MinoMold<N> {
        let matrix = matrix.map(|ary| ary.map(|n| n != 0));

        MinoMold::new(matrix)
    }
}
