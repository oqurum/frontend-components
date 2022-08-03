use std::fs;

fn main() {
    let contents = grass::from_path(
        "../../scss/main.scss",
        &grass::Options::default()
    ).expect("Unable to parse SCSS");

    fs::write(
        "./css/gcommon.css",
        contents
    ).expect("Failed writing to file");
}