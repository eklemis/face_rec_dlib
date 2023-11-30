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
fn find_distants_feature(db_path: &str, child_ids: &HashSet<String>, treshold: f64) {
    // init_progress_bar(child_ids.len());
    // set_progress_bar_action("Calculating", Color::Blue, Style::Bold);
    // finalize_progress_bar();
    let mut avg_failed = 0;
    let mut med_failed = 0;
    let mut num_rec = 0;
    for id in child_ids {
        if let Ok(fs) = FeatureSet::from_db_table(db_path, id) {
            num_rec += fs.atomics.len();
            let distant_from_avgs = fs.find_distant_atomics_from_avg(treshold);
            let distant_from_meds = fs.find_distant_atomics_from_median(treshold);
            if distant_from_avgs.len() > 0 {
                for encd in distant_from_avgs {
                    println!(
                        "From AVG:{}, {}, {}",
                        encd.child_id, encd.photo_file_name, encd.f_type
                    );
                    avg_failed += 1;
                }
            }
            if distant_from_meds.len() > 0 {
                for encd in distant_from_meds {
                    println!(
                        "From MED:{}, {}, {}",
                        encd.child_id, encd.photo_file_name, encd.f_type
                    );
                    med_failed += 1;
                }
            }
        }
    }
    println!("Total atomic record:{}", num_rec);
    println!(
        "Total Failed {} => AVG:{}, MED:{}",
        avg_failed + med_failed,
        avg_failed,
        med_failed
    );
}

fn main() {
    let db_path = String::from("dataset.db");
    let photo_path = String::from("/Users/ek_solution/Downloads/photos");

    loop {
        println!("Select an option:");
        println!("1. Extract Photos");
        println!("2. Find distant features");
        println!("3. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap(); // Make sure the prompt is displayed

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        let child_ids = extract_unique_child_ids(&photo_path);

        match choice {
            "1" => {
                extract_photos(&photo_path, &db_path, &child_ids);
            }
            "2" => {
                // Placeholder for other tasks
                println!("Enter treshold:");
                io::stdout().flush().unwrap();
                let mut choice2 = String::new();
                io::stdin().read_line(&mut choice2).unwrap();
                let treshold = choice2.parse::<f64>().unwrap_or_else(|_| 0.45);
                find_distants_feature(&db_path, &child_ids, treshold);
            }
            "3" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}
