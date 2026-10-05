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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_parses_aliases() {
        assert_eq!("v60".parse::<Method>(), Ok(Method::PourOver));
        assert_eq!("cold".parse::<Method>(), Ok(Method::ColdBrew));
        assert!("espresso".parse::<Method>().is_err());
    }
}
