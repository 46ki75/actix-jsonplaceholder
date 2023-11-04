use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Geo {
    lat: String,
    lng: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: Geo,
}

#[derive(Deserialize, Serialize, Debug)]
struct Company {
    name: String,
    #[serde(rename = "catchPhrase")]
    catch_phrase: String,
    bs: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    id: u32,
    name: String,
    username: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
    company: Company,
}
