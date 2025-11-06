use super::planet::Planet;

pub struct Uranus {}
impl Planet for Uranus {
    fn orbital_period() -> f64 {
        84.016846
    }
}
