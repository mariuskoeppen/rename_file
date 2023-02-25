use chrono::{Datelike, Timelike, Utc};
use std::fs::rename;
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting rename_file. \n Every 30 seconds the file temp.rec will be renamed to replay_DATE. ");
    
    loop {
        thread::sleep(Duration::from_millis(30_000));

        if Path::new("./temp.rec").exists() {
            let now = Utc::now();

            rename(
                Path::new("./temp.rec"),
                Path::new(&format!(
                    "./replay_{}-{:02}-{:02}_{:02}-{:02}.rec",
                    now.year(),
                    now.month(),
                    now.day(),
                    now.hour(),
                    now.minute()
                )),
            )
            .expect("An error occured renaming the file. ");

            println!("A file was renamed. ");
        }
    }
}
