/******************************************************************************
 *
 * Steganography Online Codec WebApi interface usage example.
 *
 * In this example shows how to hide an encrypted secret message in an image file.
 *
 * Version      : v1.00
 * Language     : Rust
 * Author       : Bartosz Wójcik (original example)
 * Project      : https://www.pelock.com/products/steganography-online-codec
 * Homepage     : https://www.pelock.com
 *
 * @link https://www.pelock.com/products/steganography-online-codec
 * @copyright Copyright (c) 2020-2025 PELock LLC
 * @license Apache-2.0
 *
 *****************************************************************************/

use steganography_online_codec::SteganographyOnlineCodec;

#[tokio::main]
async fn main() {
    let my_steganography_online_codec =
        SteganographyOnlineCodec::new(Some("YOUR-WEB-API-KEY".to_string()));

    let input_file = "input_file.jpg";
    let secret_message = "Secret message";
    let password = "Pa$$word";
    let output_file = "output_file_with_hidden_secret_message.png";

    match my_steganography_online_codec
        .encode(input_file, secret_message, password, output_file)
        .await
    {
        Ok(_result) => {
            println!("Secret messaged encoded and saved to the output PNG file.");
        }
        Err(err) => {
            eprintln!(
                "Encoding failed: {}",
                err.error_message()
            );
        }
    }
}
