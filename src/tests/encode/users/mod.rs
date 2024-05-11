#[cfg(feature = "surrealdb")]
mod surreal;

use fake::{Fake, Faker};
use prost::Message;

use crate::user::User;

#[test]
fn serde() {
    let user: User = Faker.fake();

    dbg!(&user);

    let json_value = serde_json::to_value(user.clone());
    dbg!(&json_value);

    assert!(json_value.is_ok());
}

#[test]
fn proto() {
    let user: User = Faker.fake();

    let other = user.clone();

    let mut buf = Vec::with_capacity(user.encoded_len());

    assert!(user.encode(&mut buf).is_ok());

    assert!(!buf.is_empty());

    let decode_result = User::decode(&*buf);
    assert!(decode_result.is_ok());

    assert_eq!(other, decode_result.unwrap());
}
