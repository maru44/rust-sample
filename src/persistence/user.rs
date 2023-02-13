// use std::fmt::Error;
use sqlx::{pool::PoolConnection, postgres::PgPool, PgConnection, Postgres};
use std::error::Error;

use crate::model::user::{User, UserInput};

// use sqlx::postgres::

// async fn users(
//     Extension(db): Extension<
// )

pub async fn create_user(
    input: UserInput,
    conn: &mut PoolConnection<Postgres>,
) -> Result<(), Error> {
    sqlx::query(
        "
        INSERT INTO users (username, email) VALUES ($1, $2);
    ",
    )
    .bind(input.username)
    .bind(input.email)
    .execute(conn)
    .await?;

    // match inserted {
    //     Err(err) => return Err(err),
    //     Ok(_) => (),
    // }
    // if let Err(err) = inserted {
    //     return Err(err);
    // }
    Ok(())
}
