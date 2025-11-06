use super::planet::Planet;

pub struct Jupiter {}
impl Planet for Jupiter {
    fn orbital_period() -> f64 {
        11.862615
    }
}
