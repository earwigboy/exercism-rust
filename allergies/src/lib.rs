pub struct Allergies {
    allergies: u32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { allergies: score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let a = allergen.clone() as u32;
        self.allergies & a == a
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        use Allergen::*;
        let allergen_list = vec![
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ];
        allergen_list
            .into_iter()
            .filter(|a| self.is_allergic_to(&a))
            .collect()
    }
}
