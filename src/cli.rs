use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "QR",
    about = "Command-line utility for reading and generating QR code"
)]
pub enum Cli {
    /// Read QR Code
    Read {
        /// Path to QR image
        #[structopt(short, long, parse(from_os_str))]
        file: PathBuf,
    },

    /// Generate QR Code
    Generate {
        /// Data to encode
        #[structopt(short, long)]
        data: String,

        /// Image format
        #[structopt(short, long, default_value = "png", possible_values = &["png", "svg"])]
        image_format: String,

        /// Output file name
        #[structopt(short, long, default_value = "qr_code")]
        file_name: String,
    },
}

impl Cli {
    pub fn get_arguments() -> Self {
        Cli::from_args()
    }
}
