use super::planet::Planet;

pub struct Earth {}
impl Planet for Earth {
    fn orbital_period() -> f64 {
        1.0
    }
}
