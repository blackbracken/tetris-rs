use derive_new::new;

pub type F32XYPos = XYPos<f32>;
pub type I16XYPos = XYPos<i16>;

#[derive(new, Copy, Clone, Debug, Default)]
pub struct XYPos<P>
where
    P: Copy + Clone,
{
    pub x: P,
    pub y: P,
}

impl<T> From<XYPos<T>> for (T, T)
where
    T: Copy + Clone,
{
    fn from(t: XYPos<T>) -> Self {
        (t.x, t.y)
    }
}

impl<T> Into<XYPos<T>> for (T, T)
where
    T: Copy + Clone,
{
    fn into(self) -> XYPos<T> {
        XYPos {
            x: self.0,
            y: self.1,
        }
    }
}
