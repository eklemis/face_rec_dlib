use crate::dbs::get_features_by_child_id;
use dlib_face_recognition::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl FeatureSet {
    pub fn from_db_table(db_path: &str, child_id: &str) -> Result<Self, String> {
        match get_features_by_child_id(&db_path, &child_id) {
            Ok(feature_set) => {
                // Use feature_set.atomics, feature_set.average, feature_set.median as needed
                return Ok(feature_set);
            }
            Err(e) => return Err(e.to_string()),
        }
    }
    fn find_distant_atomics(
        &self,
        threshold: f64,
        reference_vector: &Vec<f64>,
    ) -> Vec<FaceEncoding> {
        let mut distant_atomics = Vec::new();

        // Choose between average and median based on your requirement
        //let reference_vector = &self.average.feature_vector; // or &self.median.feature_vector

        for atomic in &self.atomics {
            let distance = euclidean_distance(&atomic.feature_vector, reference_vector);
            if distance > threshold {
                let atomic_clone: FaceEncoding = atomic.clone();
                distant_atomics.push(atomic_clone);
            }
        }

        distant_atomics
    }
    pub fn find_distant_atomics_from_avg(&self, threshold: f64) -> Vec<FaceEncoding> {
        let reference_vector = &self.average.feature_vector;
        return self.find_distant_atomics(threshold, reference_vector);
    }
    pub fn find_distant_atomics_from_median(&self, threshold: f64) -> Vec<FaceEncoding> {
        let reference_vector = &self.median.feature_vector;
        return self.find_distant_atomics(threshold, reference_vector);
    }
}
fn euclidean_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
    vec1.iter()
        .zip(vec2.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt()
}
