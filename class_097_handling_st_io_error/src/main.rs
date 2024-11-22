use std::fs;
use std::io;

fn main() {
    println!("\n\n------------------------\n\n");

    let res = rename_file("file1.txt", "file2.txt");

    if res.is_err() {
        let error = res.unwrap_err();
        eprintln!("Error kind : {:?}", error.kind());
        eprintln!("Error {}", error);
    } else {
        println!("File renamed successfully");
    }
}

fn rename_file(from: &str, to: &str) -> io::Result<()> {
    //    match fs::rename(from, to) {
    //     Ok(_) => Ok(()),
    //     Err(e) => {
    //         eprintln!("Error renaming file: {}", e);
    //         Err(e)
    //     }
    //    }
    fs::rename(from, to)?;
    Ok(())
}
