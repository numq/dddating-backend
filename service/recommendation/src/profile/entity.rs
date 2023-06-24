pub enum Gender {
    NonBinary,
    Male,
    Female,
}

pub struct Basics {
    pub age: u32,
    pub gender: Gender,
    pub location: String,
    pub preferences: Vec<Gender>,
}

pub struct Filter {
    pub min_age: u32,
    pub max_age: u32,
    pub location: String,
    pub preferences: Vec<Gender>,
}

pub struct Profile {
    pub id: String,
    pub name: String,
    pub basics: Basics,
    pub bio: String,
    pub created_at: u64,
    pub updated_at: u64,
}