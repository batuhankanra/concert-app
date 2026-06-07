use axum::{
    extract::{FromRequest, Request},
    Json,
};

use serde::de::DeserializeOwned;
use validator::Validate;

use crate::errors::error::AppError;

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(
        req: Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {

        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|_| {
                AppError::ValidationError(
                    "It cannot be empty.".to_string()
                )
            })?;

        value.validate().map_err(|err| {
            AppError::ValidationError(
                err.to_string()
            )
        })?;

        Ok(ValidatedJson(value))
    }
}