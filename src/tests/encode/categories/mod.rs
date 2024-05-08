#[cfg(feature = "surrealdb")]
mod surreal;

use fake::{Fake, Faker};
use prost::Message;

use crate::category::Category;

#[test]
fn serde() {
    let category: Category = Faker.fake();

    dbg!(&category);

    let json_value = serde_json::to_value(category.clone());
    assert!(json_value.is_ok());
}

#[test]
fn proto() {
    let category: Category = Faker.fake();

    let other = category.clone();

    let mut buf = Vec::with_capacity(category.encoded_len());

    assert!(category.encode(&mut buf).is_ok());

    assert!(!buf.is_empty());

    let decode_result = Category::decode(&*buf);
    assert!(decode_result.is_ok());

    assert_eq!(other, decode_result.unwrap());
}
