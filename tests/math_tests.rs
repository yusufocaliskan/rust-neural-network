use std::f64::consts::E;

#[test]
fn sigmoid_should_be_success() {
    let sig_result = nn::helper::math::sigmoid(2.0);
    let expected = 1.0 / (1.0 + E.powf(-2.0));
    let tolerance = 1e-9;

    dbg!(sig_result);
    dbg!(expected);
    dbg!(tolerance);

    assert!((sig_result - expected).abs() < tolerance, "Sigmoid Failed!");
}

#[test]
#[should_panic]
fn sigmoid_should_be_panic() {
    let sig_result = nn::helper::math::sigmoid(2.0);
    let expected = 1.0 / (1.0 + E.powf(2.0));
    let tolerance = 1e-9;

    dbg!(sig_result);
    dbg!(expected);
    dbg!(tolerance);

    assert!((sig_result - expected).abs() < tolerance, "Sigmoid Failed!");
}
