use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    PourOver,
    FrenchPress,
    AeroPress,
    ColdBrew,
}

impl Method {
    /// Water-to-coffee ratio, expressed as grams of water per gram of coffee.
    pub fn default_ratio(self) -> f64 {
        match self {
            Method::PourOver => 16.0,
            Method::FrenchPress => 15.0,
            Method::AeroPress => 14.0,
            Method::ColdBrew => 8.0,
        }
    }

    pub fn steep_seconds(self) -> u32 {
        match self {
            Method::PourOver => 210,
            Method::FrenchPress => 240,
            Method::AeroPress => 90,
            Method::ColdBrew => 16 * 60 * 60,
        }
    }
}

impl FromStr for Method {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_ascii_lowercase().as_str() {
            "pour-over" | "pourover" | "v60" => Ok(Method::PourOver),
            "french-press" | "press" => Ok(Method::FrenchPress),
            "aeropress" => Ok(Method::AeroPress),
            "cold-brew" | "cold" => Ok(Method::ColdBrew),
            other => Err(format!("unknown brew method: {other}")),
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Method::PourOver => "Pour-over",
            Method::FrenchPress => "French press",
            Method::AeroPress => "AeroPress",
            Method::ColdBrew => "Cold brew",
        };
        formatter.write_str(name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Recipe {
    pub method: Method,
    pub coffee_grams: f64,
    pub water_grams: f64,
}

impl Recipe {
    pub fn for_water(method: Method, water_grams: f64) -> Self {
        let coffee_grams = water_grams / method.default_ratio();
        Recipe {
            method,
            coffee_grams,
            water_grams,
        }
    }

    pub fn for_coffee(method: Method, coffee_grams: f64) -> Self {
        let water_grams = coffee_grams * method.default_ratio();
        Recipe {
            method,
            coffee_grams,
            water_grams,
        }
    }

    pub fn ratio(&self) -> f64 {
        self.water_grams / self.coffee_grams
    }
}

impl fmt::Display for Recipe {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "{}", self.method)?;
        // Coffee scales read to 0.1 g, and rounding a small AeroPress dose to
        // a whole gram shifts the ratio noticeably.
        writeln!(formatter, "  coffee: {:.1} g", self.coffee_grams)?;
        writeln!(formatter, "  water:  {:.0} g", self.water_grams)?;
        write!(formatter, "  ratio:  1:{:.1}", self.ratio())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pour_over_scales_from_water() {
        let recipe = Recipe::for_water(Method::PourOver, 320.0);
        assert_eq!(recipe.coffee_grams, 20.0);
    }

    #[test]
    fn display_keeps_tenths_of_a_gram() {
        let recipe = Recipe::for_water(Method::AeroPress, 220.0);
        assert!(recipe.to_string().contains("coffee: 15.7 g"));
    }

    #[test]
    fn method_parses_aliases() {
        assert_eq!("v60".parse::<Method>(), Ok(Method::PourOver));
        assert_eq!("cold".parse::<Method>(), Ok(Method::ColdBrew));
        assert!("espresso".parse::<Method>().is_err());
    }
}
