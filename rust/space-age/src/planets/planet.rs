use crate::duration::Duration;

pub trait Planet {
    fn orbital_period() -> f64;

    fn years_during(d: &Duration) -> f64 {
        let earth_years_in_seconds = 365.25 * 24.0 * 60.0 * 60.0;

        return d.seconds / (earth_years_in_seconds * Self::orbital_period());
    }
}
