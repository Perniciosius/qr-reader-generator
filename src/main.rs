mod cli;
mod qr;
use cli::*;
use qr::generator::*;
use qr::reader::*;
fn main() {
    let args = Cli::get_arguments();
    match args {
        Cli::Read { file } => read(file),
        Cli::Generate {
            data,
            image_format,
            file_name,
        } => generate(data, image_format, file_name),
    }
}
