use ::entity::game;
use sea_orm::{sea_query::TableCreateStatement, *};

#[cfg(feature = "test")]
pub async fn prepare_mock_db() -> DatabaseConnection {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    let schema = Schema::new(DbBackend::Sqlite);
    let stmt: TableCreateStatement = schema.create_table_from_entity(game::Entity);
    db.execute(db.get_database_backend().build(&stmt))
        .await
        .unwrap();

    game::Entity::insert_many([
        game::ActiveModel {
            id: Set(1),
            title: Set("Game A".to_owned()),
        },
        game::ActiveModel {
            id: Set(2),
            title: Set("Game B".to_owned()),
        },
        game::ActiveModel {
            id: Set(3),
            title: Set("Game C".to_owned()),
        },
    ])
    .exec(&db)
    .await
    .unwrap();

    db
}
