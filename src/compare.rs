use crate::dbs::{get_features_by_child_id, FaceEncoding};
use dlib_face_recognition::*;

pub struct Distance {
    atomics: Vec<FaceEncoding>,
    average: FaceEncoding,
    median: FaceEncoding,
}
impl Distance {
    pub fn from_db_table(db_path: &str, child_id: &str) -> Result<Self, rusqlite::Error> {
        match get_features_by_child_id(&db_path, &child_id) {
            Ok(feature_set) => {
                // Use feature_set.atomics, feature_set.average, feature_set.median as needed
                return Ok(Distance {
                    atomics: feature_set.atomics,
                    average: feature_set.average,
                    median: feature_set.median,
                });
            }
            Err(e) => return e,
        }
    }
}
pub fn compare(photo_path_1: &str, photo_path_2: &str) {}
