pub mod clap_arg{
    use std::path::PathBuf;
    use clap::Parser;
    use memmap2::Mmap;
    use anyhow::Result;
    #[derive(Parser, Debug)]
    pub struct ArgsCommand{
        #[arg(short, long)]
        pattern: String,


        #[arg(short, long)]
        directory: PathBuf,

        #[arg(short, long)]
        recursive: bool,
    }
    impl ArgsCommand{
        pub fn get_pattern(arg: &ArgsCommand) -> String{
            arg.pattern.clone()
        }
        pub fn get_directory(arg: &ArgsCommand) -> PathBuf{
            arg.directory.clone()
        }
        pub fn get_recursive(arg: &ArgsCommand) -> bool{
            arg.recursive
        }
    }
    pub fn search_in_file(arg: ArgsCommand) -> Result<()>{
        let file_path = ArgsCommand::get_directory(&arg);
        let pattern = ArgsCommand::get_pattern(&arg);
        let file = std::fs::File::open(&file_path)?;
        let mmap = unsafe{ Mmap::map(&file)?};
        let content = match std::str::from_utf8(&mmap){
            Ok(c) => c,
            Err(_) => return Ok(()),
        };
        for (i, line) in content.lines().enumerate(){
            if line.contains(&pattern){
                println!("{}:{}: {}",file_path.display(),i+1, line);
            }
        }
        Ok(())
    }
    pub fn search_in_folder(arg: ArgsCommand){

    }
}
