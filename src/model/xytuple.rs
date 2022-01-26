pub type F32XYTuple = XYTuple<f32>;

#[derive(new)]
pub struct XYTuple<P> {
    x: P,
    y: P,
}
