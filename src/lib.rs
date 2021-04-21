#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / Self::ORBITAL_PERIOD
    }
}

// using a macro reduces boilerplate code!
macro_rules! space {
    ($planet:ident, $period:expr) => {
        pub struct $planet;

        impl Planet for $planet {
            const ORBITAL_PERIOD: f64 = 31557600.0 * $period;
        }
    };
}

space!(Mercury, 0.2408467);
space!(Venus, 0.61519726);
space!(Earth, 1.0);
space!(Mars, 1.8808158);
space!(Jupiter, 11.862615);
space!(Saturn, 29.447498);
space!(Uranus, 84.016846);
space!(Neptune, 164.79132);