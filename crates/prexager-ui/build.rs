// SPDX-FileCopyrightText: 2025 Mikhail Suvorau (Suvorrow) <suvoraum@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use glob::glob;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
// use std::env;

// Import rsass functions
use rsass::{compile_scss, output::Format, output::Style};

fn main() -> Result<(), Box<dyn Error>> {
    // Get the output directory provided by Cargo
    // let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set by cargo");
    // let css_output_path = PathBuf::from(&out_dir).join("compiled_styles.css");

    // Find all .scss files
    let scss_paths: Result<Vec<PathBuf>, _> = glob("src/**/*.scss")?.collect();
    let scss_paths = scss_paths?;

    for path in &scss_paths {
        let path_str = path.to_string_lossy();
        println!("cargo:rerun-if-changed={}", path_str);
    }

    let mut scss_contents = String::from(
        "/* */
",
    );

    let mut sorted_paths = scss_paths;
    sorted_paths.sort();

    for path in &sorted_paths {
        let data = fs::read_to_string(path)?;
        scss_contents.push_str(&data);
        scss_contents.push('\n');
    }

    let format = Format {
        style: Style::Compressed,
        ..Default::default()
    };

    let css_bytes = compile_scss(scss_contents.as_bytes(), format)?;

    std::fs::create_dir_all("assets")?;

    let css_output_path = PathBuf::from("assets").join("compiled_styles.css");

    let mut output_file = File::create(&css_output_path)?;
    output_file.write_all(&css_bytes)?;
    output_file.flush()?;

    Ok(())
}
/*
// Tell Cargo to rerun script if any of the found .scss files or the script itself changes
for path in &scss_paths {
    // Convert PathBuf to &str and then to a string slice relative to current dir for cargo:rerun-if-changed
    let path_str = path.to_string_lossy();
    println!("cargo:rerun-if-changed={}", path_str);
}
// Also rerun if this build script itself changes
println!("cargo:rerun-if-changed=build.rs");

let mut scss_contents = String::from("/* */
");

    let mut sorted_paths = scss_paths;
    sorted_paths.sort();

    // Read and concatenate the content of all .scss files
    for path in &sorted_paths {
        let data = fs::read_to_string(path)?;
        scss_contents.push_str(&data);
        scss_contents.push('
');
    }

    //Define the output format (compressed, expanded, etc.)
    let format = Format {
        style: Style::Compressed,
        ..Default::default()
    };

    // Compile the combined SCSS string
    let css_bytes = compile_scss(scss_contents.as_bytes(), format)?;

    // Create and write the compiled CSS bytes to the output file
    std::fs::create_dir_all("assets")?;
    let css_output_path = PathBuf::from("assets").join("compiled_styles.css");

    let mut output_file = File::create(&css_output_path)?;
    output_file.write_all(&css_bytes)?;
    output_file.flush()?;

    Ok(())
}
    */
