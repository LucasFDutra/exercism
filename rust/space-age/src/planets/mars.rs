use super::planet::Planet;

pub struct Mars {}
impl Planet for Mars {
    fn orbital_period() -> f64 {
        1.8808158
    }
}
