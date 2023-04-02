use rusqlite::{ffi, ErrorCode};

use crate::{Connection, Result};

#[tokio::test]
async fn open_in_memory_test() -> Result<()> {
    let conn = Connection::open_in_memory().await;
    assert!(conn.is_ok());
    Ok(())
}

#[tokio::test]
async fn call_success_test() -> Result<()> {
    let conn = Connection::open_in_memory().await?;

    let result = conn
        .call(|conn| {
            conn.execute(
                "CREATE TABLE person(id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL);",
                [],
            )
        })
        .await;

    assert_eq!(0, result.unwrap());

    Ok(())
}

#[tokio::test]
async fn call_failure_test() -> Result<()> {
    let conn = Connection::open_in_memory().await?;

    let result = conn.call(|conn| conn.execute("Invalid sql", [])).await;

    assert!(match result.unwrap_err() {
        crate::Error::Rusqlite(e) => {
            e == rusqlite::Error::SqlInputError {
                error: ffi::Error {
                    code: ErrorCode::Unknown,
                    extended_code: 1,
                },
                msg: "near \"Invalid\": syntax error".to_string(),
                sql: "Invalid sql".to_string(),
                offset: 0,
            }
        }
        _ => false,
    });

    Ok(())
}

#[tokio::test]
async fn close_success_test() -> Result<()> {
    let conn = Connection::open_in_memory().await?;

    assert!(conn.close().await.is_ok());

    Ok(())
}

#[tokio::test]
async fn double_close_test() -> Result<()> {
    let conn = Connection::open_in_memory().await?;

    let conn2 = conn.clone();

    assert!(conn.close().await.is_ok());
    assert!(conn2.close().await.is_ok());

    Ok(())
}

#[tokio::test]
async fn close_call_test() -> Result<()> {
    let conn = Connection::open_in_memory().await?;

    let conn2 = conn.clone();

    assert!(conn.close().await.is_ok());

    let result = conn2.call(|conn| conn.execute("SELECT 1;", [])).await;

    assert!(matches!(
        result.unwrap_err(),
        crate::Error::ConnectionClosed
    ));

    Ok(())
}

#[tokio::test]
async fn close_failure_test() -> Result<()> {
    let conn = Connection::open_in_memory().await?;

    conn.call(|conn| {
        conn.execute(
            "CREATE TABLE person(id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL);",
            [],
        )
    })
    .await?;

    conn.call(|conn| {
        // Leak a prepared statement to make the database uncloseable
        // See https://www.sqlite.org/c3ref/close.html for details regarding this behaviour
        let stmt = Box::new(conn.prepare("INSERT INTO person VALUES (1, ?1);").unwrap());
        Box::leak(stmt);
        Ok(())
    })
    .await?;

    assert!(match conn.close().await.unwrap_err() {
        crate::Error::Close((_, e)) => {
            e == rusqlite::Error::SqliteFailure(
                ffi::Error {
                    code: ErrorCode::DatabaseBusy,
                    extended_code: 5,
                },
                Some(
                    "unable to close due to unfinalized statements or unfinished backups"
                        .to_string(),
                ),
            )
        }
        _ => false,
    });

    Ok(())
}

#[tokio::test]
async fn debug_format_test() -> Result<()> {
    let conn = Connection::open_in_memory().await?;

    assert_eq!("Connection".to_string(), format!("{:?}", conn));

    Ok(())
}

#[tokio::test]
async fn test_error_display() -> Result<()> {
    let conn = Connection::open_in_memory().await?;

    let error = crate::Error::Close((conn, rusqlite::Error::InvalidQuery));
    assert_eq!(
        "Close((Connection, \"Query is not read-only\"))",
        format!("{}", error)
    );

    let error = crate::Error::ConnectionClosed;
    assert_eq!("ConnectionClosed", format!("{}", error));

    let error = crate::Error::Rusqlite(rusqlite::Error::InvalidQuery);
    assert_eq!("Rusqlite(\"Query is not read-only\")", format!("{}", error));

    Ok(())
}

#[tokio::test]
async fn test_error_source() -> Result<()> {
    let conn = Connection::open_in_memory().await?;

    let error = crate::Error::Close((conn, rusqlite::Error::InvalidQuery));
    assert_eq!(
        std::error::Error::source(&error)
            .and_then(|e| e.downcast_ref::<rusqlite::Error>())
            .unwrap(),
        &rusqlite::Error::InvalidQuery,
    );

    let error = crate::Error::ConnectionClosed;
    assert_eq!(
        std::error::Error::source(&error).and_then(|e| e.downcast_ref::<rusqlite::Error>()),
        None,
    );

    let error = crate::Error::Rusqlite(rusqlite::Error::InvalidQuery);
    assert_eq!(
        std::error::Error::source(&error)
            .and_then(|e| e.downcast_ref::<rusqlite::Error>())
            .unwrap(),
        &rusqlite::Error::InvalidQuery,
    );

    Ok(())
}
