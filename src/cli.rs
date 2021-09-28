use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "QR", about = "Commandline utility for reading and generating QR code")]
pub enum Cli {
    /// Read QR Code
    Read {
        /// Path to QR image
        #[structopt(short, long, parse(from_os_str))]
        file: PathBuf
    },

    /// Generate QR Code
    Generate {
        /// Data to encode
        #[structopt(short, long)]
        data: String,

        /// Show output in console
        #[structopt(long, required_unless = "save")]
        show: bool,

        /// Save output to file
        #[structopt(long, required_unless = "show")]
        save: bool,

        /// Output file name
        #[structopt(short, long, default_value = "qr_code", requires = "save")]
        file_name: String
    }
}

impl Cli {
    pub fn get_arguments() -> Self {
        Cli::from_args()
    }
}