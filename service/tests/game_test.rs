mod prepare;
use entity::game;
use sea_orm::ActiveValue;
use service::{Mutation, Query};

use prepare::prepare_mock_db;

#[tokio::test]
async fn find_by_id_tests() {
    let db = &prepare_mock_db().await;

    {
        let game = Query::find_game_by_id(db, 1).await.unwrap().unwrap();

        assert_eq!(game.id, 1);
        assert_eq!(game.title, "Game A");
    }

    {
        let game = Query::find_game_by_id(db, 2).await.unwrap().unwrap();

        assert_eq!(game.id, 2);
        assert_eq!(game.title, "Game B");
    }
}

#[tokio::test]
async fn create_multiple_games() {
    let db = &prepare_mock_db().await;

    {
        let game = Mutation::create_game(
            db,
            game::Model {
                id: 0,
                title: "Game D".to_owned(),
            },
        )
        .await
        .unwrap();

        let expected_activemodel = game::ActiveModel {
            id: ActiveValue::Unchanged(4),
            title: ActiveValue::Unchanged("Game D".to_owned()),
        };
        let expected_model = game::Model {
            id: 4,
            title: "Game D".to_owned(),
        };

        assert_eq!(game, expected_activemodel);

        let game = Query::find_game_by_id(db, 4).await.unwrap().unwrap();
        assert_eq!(game, expected_model);
    }

    {
        let game = Mutation::create_game(
            db,
            game::Model {
                id: 0,
                title: "Game E".to_owned(),
            },
        )
        .await
        .unwrap();

        let expected_activemodel = game::ActiveModel {
            id: ActiveValue::Unchanged(5),
            title: ActiveValue::Unchanged("Game E".to_owned()),
        };
        let expected_model = game::Model {
            id: 5,
            title: "Game E".to_owned(),
        };

        assert_eq!(game, expected_activemodel);

        let game = Query::find_game_by_id(db, 5).await.unwrap().unwrap();
        assert_eq!(game, expected_model);
    }
    {
        let game = Mutation::create_game(
            db,
            game::Model {
                id: 0,
                title: "Game F".to_owned(),
            },
        )
        .await
        .unwrap();

        let expected_activemodel = game::ActiveModel {
            id: ActiveValue::Unchanged(6),
            title: ActiveValue::Unchanged("Game F".to_owned()),
        };
        let expected_model = game::Model {
            id: 6,
            title: "Game F".to_owned(),
        };

        assert_eq!(game, expected_activemodel);

        let game = Query::find_game_by_id(db, 6).await.unwrap().unwrap();
        assert_eq!(game, expected_model);
    }
}

#[tokio::test]
async fn update_multiple_games() {
    let db = &prepare_mock_db().await;

    {
        let game = Query::find_game_by_id(db, 1).await.unwrap().unwrap();

        assert_eq!(game.id, 1);
        assert_eq!(game.title, "Game A");

        let game = Mutation::update_game(
            db,
            1,
            game::Model {
                id: 0,
                title: "New Game A".to_owned(),
            },
        )
        .await
        .unwrap();

        assert_eq!(game.id, 1);
        assert_eq!(game.title, "New Game A");

        let game = Query::find_game_by_id(db, 1).await.unwrap().unwrap();

        assert_eq!(game.id, 1);
        assert_eq!(game.title, "New Game A");
    }

    {
        let game = Query::find_game_by_id(db, 2).await.unwrap().unwrap();

        assert_eq!(game.id, 2);
        assert_eq!(game.title, "Game B");

        let game = Mutation::update_game(
            db,
            2,
            game::Model {
                id: 0,
                title: "New Game B".to_owned(),
            },
        )
        .await
        .unwrap();

        assert_eq!(game.id, 2);
        assert_eq!(game.title, "New Game B");

        let game = Query::find_game_by_id(db, 2).await.unwrap().unwrap();

        assert_eq!(game.id, 2);
        assert_eq!(game.title, "New Game B");
    }

    {
        let game = Query::find_game_by_id(db, 3).await.unwrap().unwrap();

        assert_eq!(game.id, 3);
        assert_eq!(game.title, "Game C");

        let game = Mutation::update_game(
            db,
            3,
            game::Model {
                id: 0,
                title: "New Game C".to_owned(),
            },
        )
        .await
        .unwrap();

        assert_eq!(game.id, 3);
        assert_eq!(game.title, "New Game C");

        let game = Query::find_game_by_id(db, 3).await.unwrap().unwrap();

        assert_eq!(game.id, 3);
        assert_eq!(game.title, "New Game C");
    }
}

#[tokio::test]
async fn delete_multiple_games() {
    let db = &prepare_mock_db().await;

    {
        let game = Query::find_game_by_id(db, 1).await.unwrap().unwrap();

        assert_eq!(game.id, 1);
        assert_eq!(game.title, "Game A");

        Mutation::delete_game(db, 1).await.unwrap();

        let game_opt = Query::find_game_by_id(db, 1).await.unwrap();

        assert!(game_opt.is_none());
    }

    {
        let game = Query::find_game_by_id(db, 2).await.unwrap().unwrap();

        assert_eq!(game.id, 2);
        assert_eq!(game.title, "Game B");

        Mutation::delete_game(db, 2).await.unwrap();

        let game_opt = Query::find_game_by_id(db, 2).await.unwrap();

        assert!(game_opt.is_none());
    }

    {
        let game = Query::find_game_by_id(db, 3).await.unwrap().unwrap();

        assert_eq!(game.id, 3);
        assert_eq!(game.title, "Game C");

        Mutation::delete_game(db, 3).await.unwrap();

        let game_opt = Query::find_game_by_id(db, 3).await.unwrap();

        assert!(game_opt.is_none());
    }
}

#[tokio::test]
async fn delete_all_games() {
    let db = &prepare_mock_db().await;

    {
        let game = Query::find_game_by_id(db, 1).await.unwrap().unwrap();

        assert_eq!(game.id, 1);
        assert_eq!(game.title, "Game A");

        let game = Query::find_game_by_id(db, 2).await.unwrap().unwrap();

        assert_eq!(game.id, 2);
        assert_eq!(game.title, "Game B");

        let game = Query::find_game_by_id(db, 3).await.unwrap().unwrap();

        assert_eq!(game.id, 3);
        assert_eq!(game.title, "Game C");

        Mutation::delete_all_games(db).await.unwrap();

        let game_opt = Query::find_game_by_id(db, 1).await.unwrap();
        assert!(game_opt.is_none());
        let game_opt = Query::find_game_by_id(db, 2).await.unwrap();
        assert!(game_opt.is_none());
        let game_opt = Query::find_game_by_id(db, 3).await.unwrap();
        assert!(game_opt.is_none());
    }
}
