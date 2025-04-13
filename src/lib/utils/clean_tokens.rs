use sqlx::PgPool;

/// Function that will delete tokens that are expired.
/// When called it will run continuously and execute each 24h.
pub fn clean_tokens(pool: PgPool) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60 * 24)).await;
            match sqlx::query("DELETE FROM tokens WHERE tokens.exp < NOW()")
                .execute(&pool)
                .await
            {
                Ok(result) => {
                    println!("Deleted tokens: {}", result.rows_affected());
                }
                Err(e) => {
                    println!("Error while deleting tokens: {}", e);
                }
            }
        }
    });
}
