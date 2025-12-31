use sqlx::{Pool, Postgres};

pub async fn create_game(
    pool: Pool<Postgres>,
    name: &str,
    code: i32,
) -> eyre::Result<Option<String>> {
    let result = sqlx::query!(
        "INSERT INTO agile_game (name, code) VALUES ($1, $2) RETURNING game_id",
        name,
        code
    )
    .fetch_optional(&pool)
    .await?;

    Ok(result.map(|created_game| created_game.game_id.into()))
}
