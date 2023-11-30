use std::collections::HashSet;

use face_rec_dlib::compare::*;
use face_rec_dlib::dbs::*;
use face_rec_dlib::detect::*;
use face_rec_dlib::feature::*;
use face_rec_dlib::photos::extract_unique_child_ids;
use progress_bar::*;

use std::io::{self, Write};

fn extract_photos(photo_path: &str, db_path: &str, child_ids: &HashSet<String>) {
    init_progress_bar(child_ids.len());
    set_progress_bar_action("Extracting", Color::Blue, Style::Bold);

    match Features::new(photo_path.to_owned(), db_path.to_owned()) {
        Ok(mut fts) => {
            for id in child_ids {
                if let Err(e) = fts.process_photos(&id) {
                    print_progress_bar_info(
                        "Failed",
                        &format!("Error processing photos for child ID {}: {}", id, e),
                        Color::Red,
                        Style::Normal,
                    );
                    continue;
                }

                inc_progress_bar();
            }
            // After processing all child IDs, save any remaining features
            if !fts.get_features().is_empty() {
                if let Err(e) = fts.save_features_batch(&fts.get_features()) {
                    //eprintln!("Error saving final batch of features: {}", e);
                    print_progress_bar_info(
                        "Failed",
                        &format!("Error saving final batch of features: {}", e),
                        Color::Red,
                        Style::Normal,
                    );
                }
            }
        }
        Err(e) => {
            print_progress_bar_info(
                "Failed",
                &format!("Failed to initialize ChildFeatures: {}", e),
                Color::Red,
                Style::Normal,
            );
        }
    }
    finalize_progress_bar();
}

fn main() {
    let db_path = String::from("dataset.db");
    let photo_path = String::from("/Users/ek_solution/Downloads/photos");

    loop {
        println!("Select an option:");
        println!("1. Extract Photos");
        println!("2. Other Task (Placeholder)");
        println!("3. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap(); // Make sure the prompt is displayed

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                let child_ids = extract_unique_child_ids(&photo_path);
                extract_photos(&photo_path, &db_path, &child_ids);
            }
            "2" => {
                // Placeholder for other tasks
                println!("Other tasks to be implemented...");
            }
            "3" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}
