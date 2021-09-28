mod cli;
use cli::*;
fn main() {
    let args = Cli::get_arguments();
    match args {
        Cli::Read{ file } => {},
        Cli::Generate { data, show, save, file_name } => {}
    }
}
