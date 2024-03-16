use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::response::IntoResponse;
use axum_oidc::error::ExtractorError;
use axum_oidc::EmptyAdditionalClaims;
use axum_oidc::OidcClaims;
use tower_sessions::{
    cookie::{time::Duration, SameSite},
    Expiry, MemoryStore, SessionManagerLayer,
};

pub fn session_layer() -> SessionManagerLayer<MemoryStore> {
    let session_store = MemoryStore::default();
    SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(120)))
}

pub async fn login(_user: Option<User>) -> impl IntoResponse {
    axum::response::Redirect::to("/")
}

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
    pub email: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = <OidcClaims<EmptyAdditionalClaims> as FromRequestParts<
        OidcClaims<EmptyAdditionalClaims>,
    >>::Rejection;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        #[cfg(feature = "test")]
        // TODO: Add if
        return Ok(User {
            id: "1",
            username: "test_username",
            name: "test name",
            email: "test@test.com",
        });

        let extractor =
            OidcClaims::<EmptyAdditionalClaims>::from_request_parts(parts, state).await?;
        let id = extractor.subject().to_string();
        let username = extractor
            .preferred_username()
            .ok_or(ExtractorError::Unauthorized)?
            .to_string();
        let name = extractor
            .name()
            .ok_or(ExtractorError::Unauthorized)?
            .get(None)
            .expect("Name is not in correct Langage")
            .to_string();
        let email = extractor
            .email()
            .ok_or(ExtractorError::Unauthorized)?
            .to_string();

        Ok(User {
            id,
            username,
            name,
            email,
        })
    }
}
