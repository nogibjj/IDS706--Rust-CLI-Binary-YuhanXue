#![allow(warnings)]
#![allow(clippy::all)]
use reqwest::blocking::Client;
use rusqlite::{params, Connection, Result};
use std::fs;

use std::error::Error;
// const DATA_FILE: &str = "diabetes.csv";
const DB_FILE: &str = "diabetes.db";

pub fn load(dataset: &str) -> Result<(), Box<dyn Error>> {
    // let conn = Connection::open_in_memory()?;
    let conn = Connection::open(DB_FILE)?;
    conn.execute("DROP TABLE IF EXISTS diabetes", [])?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS diabetes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            pregnancies INTEGER,
            glucose INTEGER,
            blood_pressure INTEGER,
            skin_thickness INTEGER,
            insulin INTEGER,
            bmi DOUBLE,
            diabetes_pedigree_function DOUBLE,
            age INTEGER,
            outcome INTEGER
        )",
        [],
    )?;

    let mut rdr = csv::Reader::from_path(dataset)?;
    let mut stmt = conn.prepare(
        "INSERT INTO diabetes (pregnancies,glucose,blood_pressure,skin_thickness,insulin,bmi,diabetes_pedigree_function,age,outcome) VALUES (?,?,?,?,?,?,?,?,?)",
    )?;

    // As for why the & symbol is used, it relates to Rust's borrowing system. In the stmt.execute function, references are required to access the data within record.
    //In Rust, passing a value to a function typically transfers ownership, and to avoid transferring ownership and allow multiple references to the same data, references are used.

    for result in rdr.records() {
        match result {
            Ok(record) => {
                stmt.execute(&[
                    &record[0], &record[1], &record[2], &record[3], &record[4], &record[5],
                    &record[6], &record[7], &record[8],
                ])?;
            }
            Err(err) => {
                eprintln!("Error reading CSV record: {:?}", err);
            }
        }
    }

    Ok(())
}

pub fn read() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(DB_FILE)?;
    if let Ok(mut stmt) = conn.prepare("SELECT * FROM diabetes") {
        let results = stmt.query_map(params![], |row| {
            Ok((
                row.get::<usize, i32>(0)?,
                row.get::<usize, i32>(1)?,
                row.get::<usize, i32>(2)?,
                row.get::<usize, i32>(3)?,
                row.get::<usize, i32>(4)?,
                row.get::<usize, i32>(5)?,
                row.get::<usize, f64>(6)?,
                row.get::<usize, f64>(7)?,
                row.get::<usize, i32>(8)?,
                row.get::<usize, i32>(9)?,
            ))
        })?;

        println!("First 10 rows in diabetes table:");
        for result in results.take(10) {
            match result {
                Ok((
                    id,
                    pregnancies,
                    glucose,
                    blood_pressure,
                    skin_thickness,
                    insulin,
                    bmi,
                    diabetes_pedigree_function,
                    age,
                    outcome,
                )) => {
                    println!(
                        "Result: id={}, pregnancies={}, glucose={}, blood_pressure={}, skin_thickness={}, insulin={}, bmi={}, diabetes_pedigree_function={}, age={}, outcome={}",
                        id,
                        pregnancies,
                        glucose,
                        blood_pressure,
                        skin_thickness,
                        insulin,
                        bmi,
                        diabetes_pedigree_function,
                        age,
                        outcome
                    );
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }
    } else {
        eprintln!("Error preparing query.");
    }
    Ok(())
}

pub fn create(
    pregnancies: i32,
    glucose: i32,
    blood_pressure: i32,
    skin_thickness: i32,
    insulin: i32,
    bmi: f64,
    diabetes_pedigree_function: f64,
    age: i32,
    outcome: i32,
) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(DB_FILE)?;
    conn.execute(
        "INSERT INTO diabetes (
            pregnancies,
            glucose,
            blood_pressure,
            skin_thickness,
            insulin,
            bmi,
            diabetes_pedigree_function,
            age,
            outcome
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            pregnancies,
            glucose,
            blood_pressure,
            skin_thickness,
            insulin,
            bmi,
            diabetes_pedigree_function,
            age,
            outcome
        ],
    )?;
    Ok(())
}

pub fn update(
    id: i32,
    pregnancies: i32,
    glucose: i32,
    blood_pressure: i32,
    skin_thickness: i32,
    insulin: i32,
    bmi: f64,
    diabetes_pedigree_function: f64,
    age: i32,
    outcome: i32,
) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(DB_FILE)?;
    conn.execute(
        "UPDATE diabetes 
         SET pregnancies = ?2,
             glucose = ?3,
             blood_pressure = ?4,
             skin_thickness = ?5,
             insulin = ?6,
             bmi = ?7,
             diabetes_pedigree_function = ?8,
             age = ?9,
             outcome = ?10
        WHERE id = ?1",
        params![
            id,
            pregnancies,
            glucose,
            blood_pressure,
            skin_thickness,
            insulin,
            bmi,
            diabetes_pedigree_function,
            age,
            outcome
        ],
    )?;
    Ok(())
}

pub fn delete(id: i32) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(DB_FILE)?;

    // Check if a row with the specified ID exists
    let row_count = conn.query_row(
        "SELECT COUNT(*) FROM diabetes WHERE id = ?1",
        params![id],
        |row| row.get::<usize, i32>(0),
    )?;

    if row_count == 0 {
        // No rows found, return an error
        return Err("No row with the specified ID found".into());
    }
    // Delete the row
    conn.execute("DELETE FROM diabetes WHERE id = ?1", params![id])?;
    Ok(())
}
