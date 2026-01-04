pub mod clap_arg {
    use std::path::PathBuf;
    use memchr::{memchr_iter, memchr, memrchr, memmem};
    use clap::Parser;
    use memmap2::Mmap;
    use anyhow::Result;
    use colored::Colorize;
    use std::io::Write;
    use walkdir::{WalkDir, DirEntry}; 
    use rayon::prelude::*; 
    use std::sync::atomic::{AtomicUsize, Ordering}; 

    #[derive(Parser, Debug)]
    pub struct ArgsCommand {
        #[arg(short, long)]
        pub pattern: String, 

        #[arg(short, long)]
        pub directory: PathBuf,

        #[arg(short, long)]
        pub recursive: bool,
    }

    fn is_hidden(entry: &DirEntry) -> bool {
        entry.file_name()
             .to_str()
             .map(|s| s.starts_with('.') && s != "." && s != "..")
             .unwrap_or(false)
    }

    fn is_binary(data: &[u8]) -> bool {
        let check_len = std::cmp::min(1024, data.len());
        if memchr(b'\0', &data[..check_len]).is_some() {
            return true;
        }
        false
    }

    pub fn search_in_file(pattern: &str, file_path: &PathBuf, match_count: &AtomicUsize) -> Result<()> {
        let file = std::fs::File::open(&file_path)?;
        
        if file.metadata()?.len() == 0 {
            return Ok(());
        }

        let mmap = unsafe { Mmap::map(&file)? };

        if is_binary(&mmap) {
            return Ok(());
        }
        
        let finder = memmem::Finder::new(pattern);
        let colored_pattern = pattern.red().bold().to_string();
        let colored_bytes = colored_pattern.as_bytes();

        let colored_path = file_path.display().to_string().magenta().to_string();
        let path_bytes = colored_path.as_bytes();

        let mut search_idx = 0;
        let mut line_number = 1;
        
        let mut output_buffer = Vec::with_capacity(8 * 1024);

        while let Some(match_offset) = finder.find(&mmap[search_idx..]) {
            let absolute_match_pos = search_idx + match_offset;

            match_count.fetch_add(1, Ordering::Relaxed);

            line_number += memchr_iter(b'\n', &mmap[search_idx..absolute_match_pos]).count();

            let line_start = memrchr(b'\n', &mmap[..absolute_match_pos])
                .map(|i| i + 1)
                .unwrap_or(0);

            let line_end = memchr(b'\n', &mmap[absolute_match_pos..])
                .map(|i| absolute_match_pos + i)
                .unwrap_or(mmap.len());

            let line_bytes = &mmap[line_start..line_end];

            if std::str::from_utf8(line_bytes).is_err() {
                return Ok(());
            }

            output_buffer.extend_from_slice(path_bytes);
            output_buffer.push(b':');
            write!(output_buffer, "\x1b[32m{}\x1b[0m: ", line_number)?;

            let mut last_idx = 0;
            for sub_match in finder.find_iter(line_bytes) {
                output_buffer.extend_from_slice(&line_bytes[last_idx..sub_match]);
                output_buffer.extend_from_slice(colored_bytes);
                last_idx = sub_match + pattern.len();
            }
            output_buffer.extend_from_slice(&line_bytes[last_idx..]);
            output_buffer.push(b'\n');

            if output_buffer.len() > 8 * 1024 {
                let stdout = std::io::stdout();
                let mut handle = stdout.lock();
                handle.write_all(&output_buffer)?;
                output_buffer.clear();
            }

            if line_end == mmap.len() { break; }
            search_idx = line_end + 1;
            line_number += 1;

            if search_idx >= mmap.len() { break; }
        }

        if !output_buffer.is_empty() {
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            handle.write_all(&output_buffer)?;
        }

        Ok(())
    }

    pub fn search_in_folder(arg: &ArgsCommand, match_count: &AtomicUsize) -> Result<()> {
        if !arg.directory.exists() {
            eprintln!("Error: The directory {:?} does not exist!", arg.directory);
            return Ok(());
        }

        let entries: Vec<_> = WalkDir::new(&arg.directory)
            .into_iter()
            .filter_entry(|e| !is_hidden(e)) 
            .filter_map(|e| e.ok()) 
            .filter(|e| e.path().is_file())
            .collect();

        entries.par_iter().for_each(|entry| {
            if let Err(_e) = search_in_file(&arg.pattern, &entry.path().to_path_buf(), match_count) {
            }
        });

        Ok(())
    }
}
