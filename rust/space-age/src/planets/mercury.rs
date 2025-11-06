use super::planet::Planet;

pub struct Mercury {}
impl Planet for Mercury {
    fn orbital_period() -> f64 {
        0.2408467
    }
}
