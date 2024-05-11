use fake::{Fake, Faker};

use crate::user::User;

#[test]
fn serde() {
    let user: User = Faker.fake();

    let json_value = serde_json::to_value(user.clone());
    dbg!(&json_value);

    assert!(json_value.is_ok());
    let user_decode = serde_json::from_value::<User>(json_value.unwrap());

    dbg!(&user_decode);

    assert!(user_decode.is_ok());

    assert_eq!(user_decode.unwrap(), user);
}
