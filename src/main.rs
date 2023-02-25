use chrono::{Datelike, Timelike, Utc};
use std::fs::rename;
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        thread::sleep(Duration::from_millis(5000));

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
        }
    }
}
