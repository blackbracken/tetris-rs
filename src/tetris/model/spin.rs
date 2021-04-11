/// テトリミノの回転そのものを表現する
// TODO: 方向を持った方が良いか検討する
pub enum Spin {
    Normal,
    TSpin,
}

/// テトリミノの回転した方向を表現する
pub enum SpinDirection {
    Left,
    Right,
}