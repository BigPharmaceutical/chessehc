pub async fn add(username: &str, public_key: &[u8]) -> sqlx::Result<i64> {
    let pool = super::DB_CONNECTION
        .get()
        .expect("database is not initialised");

    let rec = sqlx::query!(
        r#"
INSERT INTO accounts ( username, public_key )
    VALUES ( $1, $2 )
    RETURNING account_id;
        "#,
        username,
        public_key
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.account_id)
}

pub async fn get_username(id: i64) -> Result<Option<String>, ()> {
    let pool = super::DB_CONNECTION
        .get()
        .expect("database is not initialised");

    match sqlx::query!(
        r#"
SELECT username
    FROM accounts
    WHERE account_id = $1;
        "#,
        id
    )
    .fetch_one(pool)
    .await
    {
        Ok(rec) => Ok(Some(rec.username)),
        Err(sqlx::Error::RowNotFound) => Ok(None),
        Err(err) => {
            eprintln!("Database Error: {err}");
            Err(())
        }
    }
}

pub async fn get_public_key(id: i64) -> Result<Option<Vec<u8>>, ()> {
    let pool = super::DB_CONNECTION
        .get()
        .expect("database is not initialised");

    match sqlx::query!(
        r#"
SELECT public_key
    FROM accounts
    WHERE account_id = $1;
        "#,
        id
    )
    .fetch_one(pool)
    .await
    {
        Ok(rec) => Ok(Some(rec.public_key)),
        Err(sqlx::Error::RowNotFound) => Ok(None),
        Err(_) => Err(()),
    }
}

pub async fn lookup(username: &str) -> Result<Option<i64>, ()> {
    let pool = super::DB_CONNECTION
        .get()
        .expect("database is not initialised");

    match sqlx::query!(
        r#"
SELECT account_id
    FROM accounts
    WHERE username = $1;
        "#,
        username
    )
    .fetch_one(pool)
    .await
    {
        Ok(rec) => Ok(Some(rec.account_id)),
        Err(sqlx::Error::RowNotFound) => Ok(None),
        Err(_) => Err(()),
    }
}

pub async fn set_username(account_id: i64, new_username: &str) -> Result<bool, ()> {
    let pool = super::DB_CONNECTION
        .get()
        .expect("database is not initialised");

    match sqlx::query!(
        r#"
UPDATE accounts
    SET username = $2
    WHERE account_id = $1;
        "#,
        account_id,
        new_username
    )
    .fetch_one(pool)
    .await
    {
        Ok(_) => Ok(true),
        Err(sqlx::Error::RowNotFound) => Ok(false),
        Err(_) => Err(()),
    }
}

pub async fn set_public_key(account_id: i64, new_public_key: &[u8]) -> Result<bool, ()> {
    let pool = super::DB_CONNECTION
        .get()
        .expect("database is not initialised");

    match sqlx::query!(
        r#"
UPDATE accounts
    SET public_key = $2
    WHERE account_id = $1;
        "#,
        account_id,
        new_public_key
    )
    .fetch_one(pool)
    .await
    {
        Ok(_) => Ok(true),
        Err(sqlx::Error::RowNotFound) => Ok(false),
        Err(_) => Err(()),
    }
}

pub async fn delete(account_id: i64) -> Result<bool, ()> {
    let pool = super::DB_CONNECTION
        .get()
        .expect("database is not initialised");

    match sqlx::query!(
        r#"
DELETE FROM accounts
    WHERE account_id = $1;
        "#,
        account_id
    )
    .execute(pool)
    .await
    {
        Ok(res) if res.rows_affected() == 1 => Ok(true),
        Ok(_) => Ok(false),
        Err(_) => Err(()),
    }
}
