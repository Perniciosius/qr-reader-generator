use std::str::from_utf8;
use quircs::Quirc;
use std::path::PathBuf;

pub fn read(path: PathBuf) {
    let img = image::open(path).expect("Failed to open image");
    let img_gray = img.into_luma8();
    let mut decoder = Quirc::default();
    let codes = decoder.identify(img_gray.width() as usize, img_gray.height() as usize, &img_gray);
    for code in codes {
        let code = code.expect("Failed to extract QR Code");
        let decoded = code.decode().expect("Failed to decode QR Code");
        println!("Data: {}", from_utf8(&decoded.payload).unwrap());
    }
}