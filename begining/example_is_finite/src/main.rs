fn main() {
    let x:f32 = 1.0/0.0;
    assert!(x.is_finite());
}
