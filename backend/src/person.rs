
use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use schema::guests;

#[table_name = "guests"]
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Person {
    pub email: String,
    pub name: String,
    pub vegetarian: String,
    pub kid: String
}

impl Person {
    pub fn create(person: Person, connection: &MysqlConnection) -> Person {
        diesel::insert_into(guests::table)
            .values(&person)
            .execute(connection)
            .expect("Error creating new person");

        guests::table.order(guests::email).first(connection).unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<Person> {
        guests::table.order(guests::email).load::<Person>(connection).unwrap()
    }
}