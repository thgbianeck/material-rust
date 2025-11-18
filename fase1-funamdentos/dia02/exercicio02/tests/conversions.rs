use exercicio02::{EscalaTemperatura, Temperatura};

const EPS: f64 = 1e-6;

#[test]
fn test_celsius_conversions() {
    let t = Temperatura::new(0.0, EscalaTemperatura::Celsius);
    assert!((t.para_fahrenheit() - 32.0).abs() < EPS);
    assert!((t.para_kelvin() - 273.15).abs() < EPS);

    let t2 = Temperatura::new(100.0, EscalaTemperatura::Celsius);
    assert!((t2.para_fahrenheit() - 212.0).abs() < EPS);
    assert!((t2.para_kelvin() - 373.15).abs() < EPS);
}

#[test]
fn test_fahrenheit_conversions() {
    let t = Temperatura::new(32.0, EscalaTemperatura::Fahrenheit);
    assert!((t.para_celsius() - 0.0).abs() < EPS);
    assert!((t.para_kelvin() - 273.15).abs() < EPS);

    let t2 = Temperatura::new(212.0, EscalaTemperatura::Fahrenheit);
    assert!((t2.para_celsius() - 100.0).abs() < EPS);
    assert!((t2.para_kelvin() - 373.15).abs() < EPS);
}

#[test]
fn test_kelvin_conversions() {
    let t = Temperatura::new(273.15, EscalaTemperatura::Kelvin);
    assert!((t.para_celsius() - 0.0).abs() < EPS);
    assert!((t.para_fahrenheit() - 32.0).abs() < EPS);

    let t2 = Temperatura::new(0.0, EscalaTemperatura::Kelvin);
    assert!((t2.para_celsius() + 273.15).abs() < EPS);
}

#[test]
fn test_converter_para_and_display() {
    let origem = Temperatura::new(25.0, EscalaTemperatura::Celsius);
    let dest = origem.converter_para(EscalaTemperatura::Fahrenheit);
    assert_eq!(dest.escala, EscalaTemperatura::Fahrenheit);
    assert!((dest.valor - 77.0).abs() < EPS);

    let origem_k = origem.converter_para(EscalaTemperatura::Kelvin);
    assert_eq!(origem_k.escala, EscalaTemperatura::Kelvin);
    assert!((origem_k.valor - 298.15).abs() < EPS);

    let t = Temperatura::new(0.0, EscalaTemperatura::Celsius);
    assert_eq!(format!("{}", t), "0.00°C");
}
