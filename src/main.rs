extern crate gtk;
extern crate zip;

use gtk::prelude::*;
use gtk::{ProgressBar, Window};
use std::time::Instant;
use std::io::{Read, Write}; // Add this line at the beginning, along with other use statements

fn zip_files(
    file_paths: Vec<&str>,
    zip_file_path: &str,
    progress_bar: gtk::ProgressBar,
) -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    let file = std::fs::File::create(zip_file_path)?;
    let mut zip = zip::ZipWriter::new(file);

    for (index, file_path) in file_paths.iter().enumerate() {
        let mut input_file = std::fs::File::open(file_path)?;
        let mut buffer = Vec::new();
        input_file.read_to_end(&mut buffer)?; // This line reads the file content into the buffer

        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        zip.start_file(file_path.to_string(), options)?; // Here, we start writing a new file to the zip archive
        zip.write_all(&buffer)?; // Write the buffer content to the zip archive

        // Update progress bar
        let progress = (index + 1) as f64 / file_paths.len() as f64;
        progress_bar.set_fraction(progress);
    }

    zip.finish()?; // Finish writing the zip archive

    let duration = start_time.elapsed();
    println!("Time taken to zip: {:?}", duration);

    Ok(())
}


fn unzip_files(
    zip_file_path: &str,
    extract_to_path: &str,
    progress_bar: gtk::ProgressBar,
) -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();

    let file = std::fs::File::open(zip_file_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let archive_len = archive.len(); // Calculate archive length here

    for i in 0..archive_len {
        let mut file = archive.by_index(i)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let output_path = format!("{}/{}", extract_to_path, file.name());

        std::fs::create_dir_all(std::path::Path::new(&output_path).parent().unwrap())?;

        let mut output_file = std::fs::File::create(output_path)?;
        output_file.write_all(&buffer)?;

        let progress = (i + 1) as f64 / archive_len as f64;
        progress_bar.set_fraction(progress);
    }

    let duration = start_time.elapsed();
    println!("Time taken: {:?}", duration);

    Ok(())
}


fn build_ui() {
    println!("Building UI...");

    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(gtk::WindowType::Toplevel);
    window.set_title("File Zipper/Unzipper");
    window.set_default_size(400, 200);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    window.add(&vbox);

    println!("UI: Added main window and vbox...");

    let format_label = gtk::Label::new(Some("Select format:"));
    vbox.pack_start(&format_label, false, false, 5);

    let file_location_label = gtk::Label::new(Some("File location:"));
    vbox.pack_start(&file_location_label, false, false, 5);

    let file_location_entry = gtk::Entry::new();
    vbox.pack_start(&file_location_entry, false, false, 5);

    let target_location_label = gtk::Label::new(Some("Target location:"));
    vbox.pack_start(&target_location_label, false, false, 5);

    let target_location_entry = gtk::Entry::new();
    vbox.pack_start(&target_location_entry, false, false, 5);

    let progress_bar = ProgressBar::new();
    vbox.pack_start(&progress_bar, false, false, 5);

    let unzip_button = gtk::Button::with_label("Unzip");
    vbox.pack_start(&unzip_button, false, false, 5);

    let zip_button = gtk::Button::with_label("Zip");
    vbox.pack_start(&zip_button, false, false, 5);

    // Cloning variables for use in closures
    let file_location_entry_clone = file_location_entry.clone();
    let target_location_entry_clone = target_location_entry.clone();
    let progress_bar_clone = progress_bar.clone();

    window.connect_destroy(|_| {
        gtk::main_quit();
    });

    unzip_button.connect_clicked(move |_| {
        println!("Unzip button clicked...");
        let file_path = file_location_entry_clone.buffer().text();
        let target_path = target_location_entry_clone.buffer().text();

        if let Err(err) = unzip_files(&file_path, &target_path, progress_bar_clone.clone()) {
            eprintln!("Error unzipping files: {}", err);
            // Optionally, show an error dialog to the user
        }
    });

    // Cloning variables for use in zip_button closure
    let file_location_entry_clone = file_location_entry.clone();
    let target_location_entry_clone = target_location_entry.clone();
    let progress_bar_clone = progress_bar.clone();

    zip_button.connect_clicked(move |_| {
        println!("Zip button clicked...");
        let file_path = file_location_entry_clone.buffer().text();
        let target_path = target_location_entry_clone.buffer().text();

        if let Err(err) = zip_files((&[file_path.as_str()])[..].to_vec(), &target_path, progress_bar_clone.clone()) {

            eprintln!("Error zipping files: {}", err);
            // Optionally, show an error dialog to the user
        }
    });

    window.show_all();

    println!("UI: Showing window...");

    gtk::main();
}


fn main() {
    build_ui();
}
