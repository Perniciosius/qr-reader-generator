use qrcode_generator::QrCodeEcc;

pub fn generate(data: String, image_format: String, file_name: String) {
    let file_name = file_name + "." + image_format.as_str();
    match image_format.as_str() {
        "png" => qrcode_generator::to_png_to_file(data, QrCodeEcc::Low, 1024, &file_name).unwrap(),
        "svg" => {
            qrcode_generator::to_svg_to_file(data, QrCodeEcc::Low, 1024, None::<&str>, &file_name)
                .unwrap()
        }
        _ => panic!("Unsupported image format"),
    }
    println!("QR saved in file: {}", file_name);
}
