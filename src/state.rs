use sea_orm::DatabaseConnection;

use crate::cli::Arguments;

#[derive(Clone)]
pub struct AppState {
    pub pool: DatabaseConnection,
    pub arguments: Arguments,
}

impl axum::extract::FromRef<AppState> for DatabaseConnection {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl axum::extract::FromRef<AppState> for Arguments {
    fn from_ref(state: &AppState) -> Self {
        state.arguments.clone()
    }
}
