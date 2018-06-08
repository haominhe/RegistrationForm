-- Your SQL goes here
CREATE TABLE guests (
    email VARCHAR(60) PRIMARY KEY NOT NULL,
    `name` VARCHAR(60) NOT NULL, 
    vegetarian VARCHAR(60) NOT NULL,
    kid VARCHAR(60) NOT NULL
)