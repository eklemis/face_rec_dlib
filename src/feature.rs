use crate::dbs::{create_face_encodings_table, insert_face_encoding};
use crate::stats::{compute_average, compute_median};
use crate::tool::{get_full_file_name, tick};
use image::ImageError;
use rayon::prelude::*;
use std::path::Path;
use walkdir::WalkDir;

use dlib_face_recognition::*;

#[derive(Debug)]
pub enum FeatureType {
    Atomic,
    Average,
    Median,
}

#[derive(Debug)]
pub struct Feature {
    child_id: String,
    feature_vector: Vec<f64>,
    photo_file_name: String,
    f_type: FeatureType,
}
impl Feature {
    pub fn from_image(
        child_id: &str,
        photo_path: &str,
        face_detector: &FaceDetectorCnn,
        landmark_predictor: &LandmarkPredictor,
        face_encoder: &FaceEncoderNetwork,
    ) -> Result<Self, String> {
        let image_buffer = image::open(photo_path)
            .map_err(|e| format!("Error opening image {}: {}", photo_path, e))?
            .to_rgb8();
        let image_matrix = ImageMatrix::from_image(&image_buffer);

        let face_locations = face_detector.face_locations(&image_matrix);
        let face_location = face_locations
            .first()
            .ok_or_else(|| "No faces detected in the image".to_string())?;

        let landmarks = landmark_predictor.face_landmarks(&image_matrix, face_location);
        let encodings = face_encoder.get_face_encodings(&image_matrix, &[landmarks], 0);
        let feature_vector = encodings
            .first()
            .ok_or_else(|| "Unable to encode face features".to_string())?
            .as_ref()
            .to_owned();

        Ok(Feature {
            child_id: child_id.to_owned(),
            feature_vector,
            photo_file_name: get_full_file_name(photo_path).to_owned(),
            f_type: FeatureType::Atomic,
        })
    }
    pub fn from_vector(
        child_id: &str,
        photo_path: &str,
        feature_vector: Vec<f64>,
        f_type: FeatureType,
    ) -> Self {
        Feature {
            child_id: child_id.to_owned(),
            feature_vector,
            photo_file_name: get_full_file_name(photo_path).to_owned(),
            f_type,
        }
    }
    pub fn save(&self, db_path: &str) -> Result<(), String> {
        insert_face_encoding(
            db_path,
            &self.child_id,
            &self.feature_vector,
            &self.photo_file_name,
            &format!("{:?}", &self.f_type),
        )
        .map_err(|e| e.to_string())
    }
}

pub struct Features {
    features: Vec<Feature>,
    photos_dir_path: String,
    db_dir_path: String,
    face_detector: FaceDetectorCnn,
    landmark_predictor: LandmarkPredictor,
    face_encoder: FaceEncoderNetwork,
}

impl Features {
    // Constructor to initialize the Features struct
    pub fn new(photos_dir_path: String, db_dir_path: String) -> Result<Self, String> {
        // Initialize the database
        Self::init_db(&db_dir_path)?;

        Ok(Features {
            features: Vec::new(),
            photos_dir_path,
            db_dir_path,
            face_detector: FaceDetectorCnn::default()?,
            landmark_predictor: LandmarkPredictor::default()?,
            face_encoder: FaceEncoderNetwork::default()?,
        })
    }
    fn init_db(db_dir_path: &str) -> Result<(), String> {
        create_face_encodings_table(db_dir_path).map_err(|e| e.to_string())
    }
    pub fn process_photos(&mut self, child_id: &str) -> Result<(), String> {
        const BATCH_SIZE: usize = 1000;
        let mut individual_feature_vectors: Vec<Vec<f64>> = Vec::new();

        // Collect eligible photo paths
        let photo_paths: Vec<_> = WalkDir::new(&self.photos_dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file() && is_target_file(e.path(), child_id))
            .filter_map(|e| e.path().to_str().map(String::from))
            .collect();

        // Sequential processing for each photo path
        for photo_path in photo_paths {
            match Feature::from_image(
                child_id,
                &photo_path,
                &self.face_detector,
                &self.landmark_predictor,
                &self.face_encoder,
            ) {
                Ok(feature) => {
                    individual_feature_vectors.push(feature.feature_vector.clone());
                    self.features.push(feature);

                    if self.features.len() >= BATCH_SIZE {
                        // Save features to the database when BATCH_SIZE is reached
                        self.save_features_batch(&self.features)?;
                        self.features.clear(); // Clear features after saving
                    }
                }
                Err(e) => eprintln!("Error processing image {}: {}", photo_path, e),
            }
        }
        // Calculate average and median feature vectors
        if !individual_feature_vectors.is_empty() {
            let average_vector = compute_average(&individual_feature_vectors);
            let median_vector = compute_median(&individual_feature_vectors);

            let average_feature =
                Feature::from_vector(child_id, "average", average_vector, FeatureType::Average);
            let median_feature =
                Feature::from_vector(child_id, "median", median_vector, FeatureType::Median);

            self.features.push(average_feature);
            self.features.push(median_feature);
        }

        // Save any remaining features (including average and median features)
        if !self.features.is_empty() {
            self.save_features_batch(&self.features)?;
            self.features.clear();
        }
        Ok(())
    }
    // Function to save a batch of features
    pub fn save_features_batch(&self, features: &[Feature]) -> Result<(), String> {
        for feature in features {
            feature.save(&self.db_dir_path)?;
        }
        Ok(())
    }
    // Function to save all features to the database using each Feature's save method
    pub fn save_features_to_db(&self) -> Result<(), String> {
        for feature in &self.features {
            feature.save(&self.db_dir_path)?;
        }

        Ok(())
    }
    pub fn get_features(&self) -> &Vec<Feature> {
        return &self.features;
    }
}
fn is_target_file(path: &Path, child_id: &str) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| {
            let lower_name = name.to_lowercase();
            (lower_name.ends_with(".jpg") || lower_name.ends_with(".png"))
                && lower_name.starts_with(child_id)
        })
        .unwrap_or(false)
}
