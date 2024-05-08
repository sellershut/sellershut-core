use fake::{Fake, Faker};

use crate::category::Category;

#[test]
fn serde() {
    let category: Category = Faker.fake();

    let json_value = serde_json::to_value(category.clone());

    assert!(json_value.is_ok());
    let cat = serde_json::from_value::<Category>(json_value.unwrap());

    assert!(cat.is_ok());

    assert_eq!(cat.unwrap(), category);
}
