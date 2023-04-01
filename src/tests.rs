use rusqlite::{ffi, Error, ErrorCode};

use crate::Connection;

#[tokio::test]
async fn open_in_memory_test() -> Result<(), rusqlite::Error> {
    let conn = Connection::open_in_memory().await;
    assert!(conn.is_ok());
    Ok(())
}

#[tokio::test]
async fn call_success_test() -> Result<(), rusqlite::Error> {
    let conn = Connection::open_in_memory().await?;

    let result = conn
        .call(|conn| {
            conn.execute(
                "CREATE TABLE person(id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL);",
                [],
            )
        })
        .await;

    assert_eq!(Ok(0), result);

    Ok(())
}

#[tokio::test]
async fn call_failure_test() -> Result<(), rusqlite::Error> {
    let conn = Connection::open_in_memory().await?;

    let result = conn.call(|conn| conn.execute("Invalid sql", [])).await;

    assert_eq!(
        Err(Error::SqlInputError {
            error: ffi::Error {
                code: ErrorCode::Unknown,
                extended_code: 1
            },
            msg: "near \"Invalid\": syntax error".to_string(),
            sql: "Invalid sql".to_string(),
            offset: 0
        }),
        result
    );

    Ok(())
}

#[tokio::test]
async fn close_success_test() -> Result<(), rusqlite::Error> {
    let conn = Connection::open_in_memory().await?;

    assert!(conn.close().await.is_ok());

    Ok(())
}

#[tokio::test]
async fn double_close_test() -> Result<(), rusqlite::Error> {
    let conn = Connection::open_in_memory().await?;

    let conn2 = conn.clone();

    assert!(conn.close().await.is_ok());
    assert!(conn2.close().await.is_ok());

    Ok(())
}

#[tokio::test]
#[should_panic]
async fn close_call_test() {
    let conn = Connection::open_in_memory().await.unwrap();

    let conn2 = conn.clone();

    assert!(conn.close().await.is_ok());
    conn2.call(|conn| conn.execute("SELECT 1;", [])).await.unwrap();
}

#[tokio::test]
async fn close_failure_test() -> Result<(), rusqlite::Error> {
    let conn = Connection::open_in_memory().await?;

    conn.call(|conn| {
        conn.execute(
            "CREATE TABLE person(id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL);",
            [],
        )
    })
    .await
    .unwrap();

    conn.call(|conn| {
        // Leak a prepared statement to make the database uncloseable
        // See https://www.sqlite.org/c3ref/close.html for details regarding this behaviour
        let stmt = Box::new(conn.prepare("INSERT INTO person VALUES (1, ?1);").unwrap());
        Box::leak(stmt);
    })
    .await;

    assert_eq!(
        Error::SqliteFailure(
            ffi::Error {
                code: ErrorCode::DatabaseBusy,
                extended_code: 5
            },
            Some("unable to close due to unfinalized statements or unfinished backups".to_string())
        ),
        conn.close().await.unwrap_err().1
    );

    Ok(())
}

#[tokio::test]
async fn debug_format_test() -> Result<(), rusqlite::Error> {
    let conn = Connection::open_in_memory().await?;

    assert_eq!("Connection".to_string(), format!("{:?}", conn));

    Ok(())
}
