type F32Location = Location<f32>;

#[derive(new)]
pub struct Location<P> {
    x: P,
    y: P,
}
