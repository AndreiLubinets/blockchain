use blockchain::{domain::block::Block, services::block::save_blocks};
use sqlx::SqlitePool;

use crate::generate_blocks;

#[sqlx::test]
async fn save_blocks_test(pool: SqlitePool) {
    let expected = generate_blocks();

    let result = save_blocks(&pool, &expected).await;
    let actual = sqlx::query_as::<_, Block>("select * from blocks")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert!(result.is_ok());
    assert_eq!(expected, actual);
}

#[sqlx::test]
async fn save_blocks_test_empty(pool: SqlitePool) {
    let expected: Vec<Block> = Vec::new();

    let result = save_blocks(&pool, &expected).await;
    let actual = sqlx::query_as::<_, Block>("select * from blocks")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert!(result.is_ok());
    assert!(actual.is_empty());
    assert_eq!(expected, actual);
}

#[sqlx::test]
async fn save_blocks_test_duplicates(pool: SqlitePool) {
    let expected = generate_blocks();

    let first_result = save_blocks(&pool, &expected).await;
    let second_result = save_blocks(&pool, &expected).await;

    let actual = sqlx::query_as::<_, Block>("select * from blocks")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert!(first_result.is_ok());
    assert!(second_result.is_ok());
    assert_eq!(expected, actual);
}
