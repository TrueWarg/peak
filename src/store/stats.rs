use std::result;

use anyhow::{Ok, Result};
use rusqlite::Connection;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
pub struct Stats {
    pub id: String,
    pub question_type: String,
    pub formatted_body: String,
    pub is_answer_right: bool,
    pub time_millis: i64,
    pub created_at_millis: i64,
}

pub fn create_table_if_not_exist(connection: &Connection) -> Result<()> {
    let query = "
      CREATE TABLE IF NOT EXISTS Stats (
        id TEXT NOT NULL PRIMARY KEY,
        question_type TEXT NOT NULL,
        formatted_body TEXT NOT NULL,
        is_answer_right INTEGER,
        time_millis INTEGER,
        created_at_millis INTEGER
      )
    ";
    connection.execute(query, ())?;
    Ok(())
}

pub fn insert_or_replace(connection: &Connection, stats: Stats) -> Result<()> {
    let query = format!(
        "
     INSERT OR REPLACE INTO Stats 
     VALUES ('{}', '{}', '{}', {}, {}, {});
  ",
        stats.id,
        stats.question_type,
        stats.formatted_body,
        if stats.is_answer_right { 1 } else { 0 },
        stats.time_millis,
        stats.created_at_millis
    );

    connection.execute(&query, ())?;
    Ok(())
}

pub fn select_all(connection: &Connection) -> Result<Vec<Stats>> {
    let query = "SELECT id, question_type, formatted_body, is_answer_right, time_millis, created_at_millis FROM Stats";
    let mut items = connection.prepare(query)?;
    let items = items.query_map([], |row| {
        result::Result::Ok(Stats {
            id: row.get(0)?,
            question_type: row.get(1)?,
            formatted_body: row.get(2)?,
            is_answer_right: row.get(3)?,
            time_millis: row.get(4)?,
            created_at_millis: row.get(5)?,
        })
    })?;
    let mut reeult: Vec<Stats> = vec![];
    for item in items {
        reeult.push(item?);
    }
    return Ok(reeult);
}
