use fake::{
    faker::internet::en::{Password, SafeEmail, Username},
    Fake,
};
use lib::{
    entity::{sea_orm_active_enums::Provider, user},
    handler::utils::hash_password,
};
use sea_orm::{prelude::Uuid, ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

pub async fn seed_one_local_user(
    db: &DatabaseConnection,
    hash_secret: &str,
) -> (user::Model, String) {
    let password: String = Password(6..25).fake();

    let hashed_password =
        hash_password(hash_secret.as_bytes(), password.as_bytes()).expect("couldn't hash password");

    let user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(Username().fake()),
        email: Set(SafeEmail().fake()),
        password_hash: Set(Some(hashed_password)),
        provider: Set(Provider::Local),
        ..Default::default()
    };

    let user = user.insert(db).await.expect("couldn't insert user");

    (user, password)
}
