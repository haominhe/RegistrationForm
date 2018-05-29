
#[derive(Serialize, Deserialize)]
pub enum Family {
    Object(Map<String, Head>)
    //RANDOM STRING: Head
}

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