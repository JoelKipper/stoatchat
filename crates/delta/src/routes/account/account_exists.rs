//! Check whether an account exists for a given email
//! POST /account/exists
use rocket::serde::json::Json;
use rocket::State;
use revolt_result::Result;
use revolt_database::{Database, util::email::{normalise_email, validate_email}};
use revolt_models::v0;

/// # Check Account Exists
///
/// Check whether an account is registered for a given email, so the client
/// can show a login or a registration form without the user picking one
/// up front.
#[openapi(tag = "Account")]
#[post("/exists", data = "<data>")]
pub async fn account_exists(
    db: &State<Database>,
    data: Json<v0::DataAccountExists>,
) -> Result<Json<v0::AccountExists>> {
    let data = data.into_inner();

    // Make sure email is valid and not blocked
    validate_email(&data.email)?;

    // Normalise the email
    let email_normalised = normalise_email(data.email);

    // Try to find the relevant account
    let exists = db
        .fetch_account_by_normalised_email(&email_normalised)
        .await?
        .is_some();

    Ok(Json(v0::AccountExists { exists }))
}
