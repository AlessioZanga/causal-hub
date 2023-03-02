/// Map NaN to zero.
#[inline]
pub fn nan_to_zero(x: f64) -> f64 {
    if x.is_nan() {
        0.
    } else {
        x
    }
}
