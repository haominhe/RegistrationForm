
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Family(HashMap<String, Head>);

#[derive(Serialize, Deserialize)]
pub struct Head {
    pub name: String,
    pub vegetarian: bool,
    pub kid: bool,
    pub familyMembers: Vec<Person>
}

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub vegetarian: bool,
    pub kid: bool
}