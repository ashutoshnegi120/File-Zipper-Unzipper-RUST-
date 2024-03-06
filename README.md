# File Zipper/Unzipper Rust Application

This Rust application provides a graphical user interface (GUI) for zipping and unzipping files. It utilizes the GTK toolkit for creating the user interface and the `zip` crate for handling zip file operations.

## Features

- Zip files: Select multiple files and zip them into a single zip archive.
- Unzip files: Select a zip archive and extract its contents to a specified location.
- Progress bar: Visual indication of the progress for zip/unzip operations.

## Usage

1. **Select format**: Choose between "Zip" or "Unzip".
2. **File location**: Provide the path to the file(s) you want to zip or the zip archive you want to unzip.
3. **Target location**: Specify the location where the zip archive will be created (for zipping) or where the contents of the zip archive will be extracted (for unzipping).
4. **Click Zip/Unzip**: Click the corresponding button to initiate the zip or unzip operation.

## Dependencies

- `gtk`: The GTK toolkit for creating the graphical user interface.
- `zip`: A crate for creating and extracting zip archives.

## Build and Run

1. Install Rust and Cargo (if not already installed).
2. Clone the repository: `git clone https://github.com/ashutoshnegi120/file-zipper-unzipper-rust-.git`
3. Navigate to the project directory: `cd file-zipper-unzipper-rust-`
4. Build the project: `cargo build --release`
5. Run the application: `cargo run --release`

## Screenshots

![Screenshot 1](screenshots/screenshot1.png)
![Screenshot 2](screenshots/screenshot2.png)

## Contributions

Contributions are welcome! If you find any bugs or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
