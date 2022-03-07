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

/*
impl<P> XYPos<P>
where
    P: Copy + Clone,
{
    fn of<Q: Into<P>>(x: Q, y: Q) -> Self {
        XYPos::new(x.into(), y.into())
    }
}
*/

impl<T> From<XYPos<T>> for (T, T)
where
    T: Copy + Clone,
{
    fn from(pos: XYPos<T>) -> Self {
        (pos.x, pos.y)
    }
}

impl<T> From<(T, T)> for XYPos<T>
where
    T: Copy + Clone,
{
    fn from(tuple: (T, T)) -> Self {
        let (x, y) = tuple;
        XYPos::new(x, y)
    }
}
