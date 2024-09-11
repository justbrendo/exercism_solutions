// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

static SECONDS_PER_YEAR: f64 = 31557600f64;

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {seconds: s}
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! planet_impl {
    ($p:ident, $r:expr) => {
        pub struct $p;

        impl Planet for $p {
            fn years_during(d: &Duration) -> f64 {
                let ratio_to_earth = 1f64 / $r;
                let earthy_years = d.seconds as f64 / SECONDS_PER_YEAR;
                ratio_to_earth * earthy_years
            }
        }
    };
}

planet_impl!(Mercury, 0.2408467);
planet_impl!(Venus, 0.61519726);
planet_impl!(Earth, 1.0);
planet_impl!(Mars, 1.8808158);
planet_impl!(Jupiter, 11.862615);
planet_impl!(Saturn, 29.447498);
planet_impl!(Uranus, 84.016846);
planet_impl!(Neptune, 164.79132);