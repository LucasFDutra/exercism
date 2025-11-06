use super::planet::Planet;

pub struct Saturn {}
impl Planet for Saturn {
    fn orbital_period() -> f64 {
        29.447498
    }
}
