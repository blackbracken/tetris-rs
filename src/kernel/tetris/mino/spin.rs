use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

/// 回転の方向を表現する.
pub enum SpinDirection {
    /// 右回り
    Clockwise,

    /// 左回り
    CounterClockwise,
}

/// テトリミノの回転した角度を表現する.
/// 初期位置で12時の方向を示し, 時計回りに回転する.
#[derive(FromPrimitive, ToPrimitive, PartialEq, Eq, Debug)]
pub enum MinoRotation {
    Clockwise = 0,
    Clockwise90 = 90,
    Clockwise180 = 180,
    Clockwise270 = 270,
}

impl MinoRotation {
    pub fn spin(&self, direction: &SpinDirection) -> MinoRotation {
        match direction {
            SpinDirection::CounterClockwise => self.left(),
            SpinDirection::Clockwise => self.right(),
        }
    }

    fn left(&self) -> MinoRotation {
        self.rotate(270).unwrap()
    }

    fn right(&self) -> MinoRotation {
        self.rotate(90).unwrap()
    }

    fn rotate(&self, add: usize) -> Option<MinoRotation> {
        let angle = ToPrimitive::to_usize(self).unwrap();

        FromPrimitive::from_usize((angle + add) % 360)
    }
}

impl Default for MinoRotation {
    fn default() -> Self {
        MinoRotation::Clockwise
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(SpinDirection::Clockwise, MinoRotation::Clockwise90)]
    #[test_case(SpinDirection::CounterClockwise, MinoRotation::Clockwise270)]
    fn rotate(direction: SpinDirection, ans: MinoRotation) {
        let rot = MinoRotation::Clockwise;
        let rot = rot.spin(&direction);

        assert_eq!(rot, ans);
    }
}
