use crate::query::Query;
use ::entity::{game, game::Entity as Game};
use sea_orm::*;

impl Query {
    pub async fn find_game_by_id(db: &DbConn, id: i32) -> Result<Option<game::Model>, DbErr> {
        Game::find_by_id(id).one(db).await
    }
}
