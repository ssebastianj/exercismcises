// TODO: This exercise is not finished.

#[derive(PartialEq, Debug, Clone, Copy)]
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

struct Allergies(pub u32);

impl Allergies {
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        // TODO: Implement this method
        true
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let person_allergies: Vec<Allergen> = vec![];
        // TODO: Implement this method
        person_allergies
    }
}
