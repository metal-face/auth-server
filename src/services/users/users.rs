use crate::api::v1::users::users::UserDTO;
use crate::repositories::users::users::{create_user, User};
use anyhow::bail;
use email_address::EmailAddress;
use sqlx::PgPool;

pub async fn validate_user(user: UserDTO, pool: &PgPool) -> anyhow::Result<User, anyhow::Error> {
    let UserDTO {
        ref first_name,
        ref last_name,
        ..
    } = user;

    if user.password.chars().count() < 12 {
        bail!("User password must be at least 12 characters");
    }

    let is_valid_email_address = EmailAddress::is_valid(&user.email);

    if !is_valid_email_address {
        bail!("Invalid email address");
    }

    if first_name.chars().count() < 3 || first_name.chars().count() > 24 {
        bail!("User first name length is invalid");
    }

    if last_name.chars().count() < 3 || last_name.chars().count() > 24 {
        bail!("User last name length is invalid");
    }

    let res = create_user(pool, user).await?;

    Ok(res)
}
