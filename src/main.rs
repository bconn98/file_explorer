//extern crate gtk;
extern crate chrono;

use chrono::prelude::DateTime;
//use gtk::prelude::*;
use chrono::Utc;
use std::fs;

mod structs;

fn main() -> std::io::Result<()> {
    let path = "/samba/Documents/Courses";
    let depth = 0;
    read_interior(depth, path)
}

fn read_file_info(file: fs::DirEntry, metadata: fs::Metadata, depth: usize) {
    if let Ok(last_accessed) = metadata.modified() {
        // Create DateTime from SystemTime
        let datetime = DateTime::<Utc>::from(last_accessed);
        // Formats the combined date and time with the specified format string.
        let last_accessed = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();

        let name = file.file_name().into_string().unwrap();
        let size = metadata.len();
        let path = file.path();
        let f = structs::make_file_info(name, last_accessed, size, path);

        //println!("{:?}", f);
    } else {
        // ignore
    }

    let dir_name_str = format!("|--- {}", file.file_name().to_str().unwrap());
    println!("{}", add_chars("  ", &*dir_name_str, depth));
}

fn add_chars(repeat_val: &str, txt: &str, copies: usize) -> String {
    let mut repeated: String = repeat_val.repeat(copies);
    repeated.push_str(txt);
    repeated
}

fn read_dir_info(dir: fs::DirEntry, metadata: fs::Metadata, depth: usize) -> fs::DirEntry {
    if let Ok(last_accessed) = metadata.modified() {
        // Create DateTime from SystemTime
        let datetime = DateTime::<Utc>::from(last_accessed);
        // Formats the combined date and time with the specified format string.
        let last_accessed = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();

        let name = dir.file_name().into_string().unwrap();
        let size = metadata.len();
        let path = dir.path();
        let f = structs::make_dir_info(name, last_accessed, size, path);

        //println!("{:?}", f);
    } else {
        // ignore
    }

    let dir_name_str = format!("|--- {}", dir.file_name().to_str().unwrap());
    println!("{}", add_chars("   ", &*dir_name_str, depth));

    dir
}

fn read_interior(depth: usize, path: &str) -> std::io::Result<()> {
    //let bar_line = "|---";
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_file() {
            read_file_info(entry, metadata, depth + 1);
        }
        else if metadata.is_dir() {
            let entry = read_dir_info(entry, metadata, depth);
            let _ = read_interior(depth + 1, entry.path().to_str().unwrap());
        }
    }
    Ok(())
}

/*fn start_gui() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("../xml_config/main2.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.get_object("window1").unwrap();
    //let button: gtk::Button = builder.get_object("button1").unwrap();
    //let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();

    //button.connect_clicked(move |_| {
    //    dialog.run();
    //    dialog.hide();
    //});

    window.show_all();

    // End program on exit of window
    window.connect_delete_event(|_,_| {gtk::main_quit(); Inhibit(false) });

    gtk::main();
}
*/
