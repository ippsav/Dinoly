use fake::{
    faker::{
        internet::en::{DomainSuffix, Username},
        lorem::en::Word,
    },
    Fake,
};
use lib::entity::url;
use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection,
    EntityTrait, QueryFilter,
};

pub async fn seed_links_for_user(db: &DatabaseConnection, user_id: &Uuid) -> Vec<url::Model> {
    let link_models = (0..10).map(|_| url::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(Username().fake()),
        slug: Set(Word().fake()),
        redirect_to: Set(format!(
            "https://{}.{}",
            Word().fake::<String>(),
            DomainSuffix().fake::<String>()
        )),
        owner_id: Set(user_id.clone()),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    });
    url::Entity::insert_many(link_models)
        .exec(db)
        .await
        .expect("couldn't insert all links");

    url::Entity::find()
        .filter(url::Column::OwnerId.eq(user_id.clone()))
        .all(db)
        .await
        .expect("couldn't get all links")
}

pub async fn seed_one_link_for_user(db: &DatabaseConnection, user_id: &Uuid) -> url::Model {
    let link_model = url::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(Username().fake()),
        slug: Set(Word().fake()),
        redirect_to: Set(format!(
            "https://{}.{}",
            Word().fake::<String>(),
            DomainSuffix().fake::<String>()
        )),
        owner_id: Set(user_id.clone()),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    link_model.insert(db).await.expect("couldn't insert link")
}
