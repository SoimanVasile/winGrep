use clap::Parser;
use crate::clap_arg::clap_arg::{ArgsCommand, search_in_file, search_in_folder};
// 1. Import Atomic Counter
use std::sync::atomic::{AtomicUsize, Ordering};

mod clap_arg;

fn main() {
    let arg = ArgsCommand::parse();

    // 2. Global match counter (Thread-safe)
    let match_count = AtomicUsize::new(0);

    if arg.recursive {
        let _ = search_in_folder(&arg, &match_count);
    } else {
        // Direct field access (No more getters!)
        let file_path = &arg.directory;
        let pattern = &arg.pattern;
        
        if file_path.is_file() {
            let _ = search_in_file(pattern, file_path, &match_count);
        } else {
            println!("Is not a file which i can search for, maybe put the -r flag");
        }
    }

    // 3. Print Summary
    let count = match_count.load(Ordering::Relaxed);
    if count > 0 {
        println!("\nFound {} matches.", count);
    }
}
