use std::env;
use qrcode_generator::QrCodeEcc;

fn main() {
    let args: Vec<String> = env::args().collect();

    let encoded_string = &args[1];
    let filename = &args[2];
    let fileending = ".png";
    let file = format!("{}{}", filename, fileending);

    println!("Encoding: {:?}", encoded_string);
    println!("In file: {:?}", file);
    qrcode_generator::to_png_to_file_from_str(encoded_string, QrCodeEcc::High, 512, file).unwrap();
}
