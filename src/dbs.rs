use bincode; // For serialization
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FaceEncoding {
    pub id: i32,
    pub child_id: String,
    pub feature_vector: Vec<f64>,
    pub photo_file_name: String,
    pub f_type: String,
    pub timestamp: String,
}
pub struct FeatureSet {
    pub atomics: Vec<FaceEncoding>,
    pub average: FaceEncoding,
    pub median: FaceEncoding,
}

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

pub fn get_features_by_child_id(db_path: &str, child_id: &str) -> Result<FeatureSet> {
    let conn = Connection::open(db_path)?;

    let mut stmt = conn.prepare(
        "SELECT id, childID, featureVector, photoFileName, type, timestamp
         FROM FaceEncodings
         WHERE childID = ?1",
    )?;

    let mut atomics: Vec<FaceEncoding> = Vec::new();
    let mut average: Option<FaceEncoding> = None;
    let mut median: Option<FaceEncoding> = None;

    let face_encoding_iter = stmt.query_map(params![child_id], |row| {
        Ok(FaceEncoding {
            id: row.get(0)?,
            child_id: row.get(1)?,
            feature_vector: bincode::deserialize(&row.get::<_, Vec<u8>>(2)?)?,
            photo_file_name: row.get(3)?,
            f_type: row.get(4)?,
            timestamp: row.get(5)?,
        })
    })?;

    for encoding in face_encoding_iter {
        let encoding = encoding?;
        match encoding.f_type.as_str() {
            "Atomic" => atomics.push(encoding),
            "Average" => average = Some(encoding),
            "Median" => median = Some(encoding),
            _ => {} // Handle unexpected type or do nothing
        }
    }

    // Ensure that both average and median features are present
    let average = average.ok_or_else(|| rusqlite::Error::QueryReturnedNoRows)?;
    let median = median.ok_or_else(|| rusqlite::Error::QueryReturnedNoRows)?;

    Ok(FeatureSet {
        atomics,
        average,
        median,
    })
}
