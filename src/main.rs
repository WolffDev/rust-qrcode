use base64;
use brotli::{enc::backward_references::BrotliEncoderParams, BrotliCompress, BrotliDecompress};
use qrcode_generator::QrCodeEcc;
use std::env;
use std::io::Cursor;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "decode" {
        match decompress_to_string(&args[2]) {
            Ok(s) => println!("{}", s),
            Err(e) => println!("Error: {}", e),
        }
        return;
    }

    if args.len() != 3 {
        println!("Usage: qr-gen <string> <filename>");
        return;
    }

    let encoded_string = &args[1];
    let compressed_string = compress_string(encoded_string);
    let filename = &args[2];
    let fileending = ".svg";
    let file = format!("{}{}", filename, fileending);
    let desc = "QR Code";

    println!("Encoding: {}", encoded_string);
    println!("Compressed: {}", compressed_string);
    println!("In file: {}", file);
    qrcode_generator::to_svg_to_file_from_str(
        compressed_string,
        QrCodeEcc::High,
        1024,
        Some(desc),
        file,
    )
    .unwrap();
}

fn compress_string(input: &str) -> String {
    let mut params = BrotliEncoderParams::default();
    params.quality = 11; // Set the compression quality (0-11, where 11 is the highest)

    let mut compressed_data = Vec::new();
    let mut input_data = Cursor::new(input.as_bytes());
    BrotliCompress(&mut input_data, &mut compressed_data, &params).unwrap();

    // Encode the compressed data in Base64
    base64::encode(&compressed_data)
}

fn decompress_to_string(
    encoded_compressed_data: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Decode the Base64-encoded compressed data
    let compressed_data = base64::decode(encoded_compressed_data)?;

    let mut decompressed_data = Vec::new();
    let mut compressed_data_cursor = Cursor::new(&compressed_data);
    BrotliDecompress(&mut compressed_data_cursor, &mut decompressed_data).unwrap();

    let decompressed_string = String::from_utf8(decompressed_data)?;
    Ok(decompressed_string)
}
