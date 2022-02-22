use derive_new::new;

pub type F32XYTuple = XYTuple<f32>;

#[derive(new, Copy, Clone, Debug, Default)]
pub struct XYTuple<P>
where
    P: Copy + Clone,
{
    pub x: P,
    pub y: P,
}

impl<T> From<XYTuple<T>> for (T, T)
where
    T: Copy + Clone,
{
    fn from(t: XYTuple<T>) -> Self {
        (t.x, t.y)
    }
}

impl<T> Into<XYTuple<T>> for (T, T)
where
    T: Copy + Clone,
{
    fn into(self) -> XYTuple<T> {
        XYTuple {
            x: self.0,
            y: self.1,
        }
    }
}
