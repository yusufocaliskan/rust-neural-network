/// a small helper, just for a clean code
/// * trc - tolerance
pub fn debug_helper(a: f64, b: f64, trc: f64) -> bool {
    (a - b).abs() < trc
}

//Quick printing
pub fn pr<T: std::fmt::Debug>(text: &str, data: T) {
    //On debug mode
    if cfg!(debug_assertions) {
        println!("{} --> {:#?}", text, data);
    }
}
