use std::fmt;

use crate::method::Method;

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
        writeln!(formatter, "  coffee: {:.0} g", self.coffee_grams)?;
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
}
