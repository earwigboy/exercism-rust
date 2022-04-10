const EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64)
    }
}

pub trait Planet {
    const YEAR_IN_SECONDS: f64;
    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::YEAR_IN_SECONDS
    }
}

macro_rules! define_planet {
    (name = $name:ident, orbit = $orbit:expr) => {
        pub struct $name;

        impl Planet for $name {
            const YEAR_IN_SECONDS: f64 = $orbit * EARTH_YEAR_IN_SECONDS;
        }
    };
}

define_planet!(name = Mercury, orbit = 0.2408467);
define_planet!(name = Venus, orbit = 0.61519726);
define_planet!(name = Earth, orbit = 1.0);
define_planet!(name = Mars, orbit = 1.8808158);
define_planet!(name = Jupiter, orbit = 11.862615);
define_planet!(name = Saturn, orbit = 29.447498);
define_planet!(name = Uranus, orbit = 84.016846);
define_planet!(name = Neptune, orbit = 164.79132);
