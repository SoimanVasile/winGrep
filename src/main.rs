use clap::Parser;
use std::fs;
use std::path::PathBuf;

use crate::clap_arg::clap_arg::search_in_file;

mod clap_arg;
fn main(){
    let arg = clap_arg::clap_arg::ArgsCommand::parse();
    search_in_file(arg);

}
