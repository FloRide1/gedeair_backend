use crate::mutation::Mutation;
use ::entity::{game, game::Entity as Game};
use sea_orm::*;

impl Mutation {
    pub async fn create_game(
        db: &DbConn,
        form_data: game::Model,
    ) -> Result<game::ActiveModel, DbErr> {
        game::ActiveModel {
            id: NotSet,
            title: Set(form_data.title.to_owned()),
        }
        .save(db)
        .await
    }

    pub async fn update_game(
        db: &DbConn,
        id: i32,
        form_data: game::Model,
    ) -> Result<game::Model, DbErr> {
        let game: game::ActiveModel = Game::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find Game.".to_owned()))
            .map(Into::into)?;

        game::ActiveModel {
            id: game.id,
            title: Set(form_data.title.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_game(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let game: game::ActiveModel = Game::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find Game.".to_owned()))
            .map(Into::into)?;

        game.delete(db).await
    }

    pub async fn delete_all_games(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Game::delete_many().exec(db).await
    }
}
