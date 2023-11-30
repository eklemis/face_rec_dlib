use crate::compare::{FaceEncoding, FeatureSet};
use crate::error::*;
use bincode; // For serialization
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::io;

pub fn create_face_encodings_table(db_path: &str) -> Result<()> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS FaceEncodings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            childID TEXT NOT NULL,
            featureVector BLOB NOT NULL,
            photoFileName TEXT,
            type TEXT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        );",
        [],
    )?;

    Ok(())
}

pub fn insert_face_encoding(
    db_path: &str,
    child_id: &str,
    feature_vector: &Vec<f64>,
    photo_file_name: &str,
    f_type: &str,
) -> Result<()> {
    let conn = Connection::open(db_path)?;

    // Serialize the Vec<f64> into a byte array
    let serialized_feature_vector = match bincode::serialize(feature_vector) {
        Ok(vec) => vec,
        Err(e) => return Err(rusqlite::Error::ExecuteReturnedResults), // Or handle differently
    };

    conn.execute(
        "INSERT INTO FaceEncodings (childID, featureVector, photoFileName, type) VALUES (?1, ?2, ?3, ?4)",
        params![child_id, serialized_feature_vector, photo_file_name, f_type],
    )?;

    Ok(())
}

pub fn get_features_by_child_id(db_path: &str, child_id: &str) -> Result<FeatureSet, AppError> {
    let conn = Connection::open(db_path).map_err(AppError::Sqlite)?;

    let mut stmt = conn
        .prepare(
            "SELECT id, childID, featureVector, photoFileName, type, timestamp
         FROM FaceEncodings
         WHERE childID = ?1",
        )
        .map_err(AppError::Sqlite)?;

    let mut atomics: Vec<FaceEncoding> = Vec::new();
    let mut average: Option<FaceEncoding> = None;
    let mut median: Option<FaceEncoding> = None;

    let face_encoding_iter = stmt
        .query_map(params![child_id], |row| {
            let feature_vector_blob: Vec<u8> = row.get(2)?;

            let feature_vector = match bincode::deserialize(&feature_vector_blob) {
                Ok(vec) => vec,
                Err(_) => return Err(rusqlite::Error::InvalidQuery), // Using a generic error
            };

            Ok(FaceEncoding {
                id: row.get(0)?,
                child_id: row.get(1)?,
                feature_vector,
                photo_file_name: row.get(3)?,
                f_type: row.get(4)?,
                timestamp: row.get(5)?,
            })
        })
        .map_err(AppError::Sqlite)?;

    for encoding in face_encoding_iter {
        let encoding = encoding.map_err(AppError::Sqlite)?;
        match encoding.f_type.as_str() {
            "Atomic" => atomics.push(encoding),
            "Average" => average = Some(encoding),
            "Median" => median = Some(encoding),
            _ => {}
        }
    }

    let average = average
        .ok_or_else(|| AppError::Io(io::Error::new(io::ErrorKind::NotFound, "No average found")))?;
    let median = median
        .ok_or_else(|| AppError::Io(io::Error::new(io::ErrorKind::NotFound, "No median found")))?;

    Ok(FeatureSet {
        atomics,
        average,
        median,
    })
}
