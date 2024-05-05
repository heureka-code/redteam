use std::sync::Arc;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct RequestPettable {
    token: Arc<str>,
    name_pattern: Arc<str>,
}

impl RequestPettable {
    pub fn new(token: impl Into<Arc<str>>, name_pattern: impl Into<Arc<str>>) -> Self {
        Self {
            token: token.into(),
            name_pattern: name_pattern.into(),
        }
    }
    pub fn token(&self) -> &str {
        &self.token
    }
    pub fn name_pattern(&self) -> &str {
        &self.name_pattern
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct Pet {
    name: Arc<str>,
    pettype: Arc<str>,
}

impl Pet {
    pub fn new(name: impl Into<Arc<str>>, pettype: impl Into<Arc<str>>) -> Self {
        Self {
            name: name.into(),
            pettype: pettype.into(),
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn pettype(&self) -> &str {
        &self.pettype
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ResponsePettable {
    name_pattern: Arc<str>,
    pets: Vec<Pet>,
}

impl ResponsePettable {
    pub fn new(name_pattern: impl Into<Arc<str>>, pets: impl Into<Vec<Pet>>) -> Self {
        Self {
            name_pattern: name_pattern.into(),
            pets: pets.into(),
        }
    }
    pub fn pets(&self) -> &[Pet] {
        &self.pets
    }
    pub fn owned_pets(&self) -> Vec<Pet> {
        self.pets.clone()
    }
}
