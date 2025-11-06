use super::planet::Planet;

pub struct Venus {}
impl Planet for Venus {
    fn orbital_period() -> f64 {
        0.61519726
    }
}
