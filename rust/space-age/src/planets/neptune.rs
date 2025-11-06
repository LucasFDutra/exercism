use super::planet::Planet;

pub struct Neptune {}
impl Planet for Neptune {
    fn orbital_period() -> f64 {
        164.79132
    }
}
