use std::path::Path;

fn main() {
    println!("{}", Path::new("./temp.rec").exists());
}