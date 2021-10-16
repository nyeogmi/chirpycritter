pub(crate) fn lerp(amt: f32, x0: f32, x1: f32) -> f32 {
    if amt <= 0.0 { return x0; }
    if amt >= 1.0 { return x1; }
    return x0 + (x1 - x0) * amt;
}