use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Allergies {
     allergies:Vec<Allergen>
}

#[derive(Debug, PartialEq, Eq, EnumIter, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {

    pub fn new(score: u32) -> Self {        
        let mut allergies = Vec::new();

        for (i, x) in Allergen::iter().enumerate() {
            if score & (1 << i) != 0 {
                allergies.push(x);
            }
        }

        Self { allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}
