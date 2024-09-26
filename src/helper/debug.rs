/// a small helper, just for a clean code
/// * trc - tolerance
pub fn debug_helper(a: f64, b: f64, trc: f64) -> bool {
    (a - b).abs() < trc
}
