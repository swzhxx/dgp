use plotters::prelude::*;
fn main() {
    let area = BitMapBackend::gif("images/animated.gif", (320, 100), 1000)
        .unwrap()
        .into_drawing_area();
    for i in 0..=10 {
        area.fill(&WHITE).unwrap();
        area.draw(&Text::new(
            format!("{}", 10 - i),
            (100, 20),
            ("sans-serif", 80),
        ))
        .unwrap();
        area.present().unwrap();
    }
}
