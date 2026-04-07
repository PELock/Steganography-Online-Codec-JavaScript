/******************************************************************************
 *
 * Steganography Online Codec WebApi interface usage example.
 *
 * In this example, we will see how to hide an encrypted message in an
 * image file using our codec.
 *
 * Version      : v1.00
 * Language     : Rust
 * Author       : Bartosz Wójcik
 * Project      : https://www.pelock.com/products/steganography-online-codec
 * Homepage     : https://www.pelock.com
 *
 * @link https://www.pelock.com/products/steganography-online-codec
 * @copyright Copyright (c) 2020-2025 PELock LLC
 * @license Apache-2.0
 *
 *****************************************************************************/

use steganography_online_codec::{errors, SteganographyOnlineCodec};

#[tokio::main]
async fn main() {
    let my_steganography_online_codec =
        SteganographyOnlineCodec::new(Some("YOUR-WEB-API-KEY".to_string()));

    let input_file_path = "input_file.webp";
    let secret_message = "Secret message";
    let password = "Pa$$word";
    let output_file_path = "output_file_with_hidden_secret_message.png";

    match my_steganography_online_codec
        .encode(
            input_file_path,
            secret_message,
            password,
            output_file_path,
        )
        .await
    {
        Ok(result) => {
            let version_type = result
                .license
                .as_ref()
                .and_then(|l| l.activation_status)
                .unwrap_or(false);
            println!(
                "You are running in {} version",
                if version_type { "full" } else { "demo" }
            );

            println!(
                "Secret messaged encoded and saved to {}",
                output_file_path
            );
            if let Some(ref lic) = result.license {
                if let Some(c) = lic.usages_count {
                    println!("Remaining number of usage credits - {c}");
                }
            }
        }
        Err(err) => {
            let error_code = err.code();
            match error_code {
                errors::INVALID_INPUT => {
                    println!(
                        "Invalid input file {} or file doesn't exist",
                        input_file_path
                    );
                }
                errors::MESSAGE_TOO_LONG => {
                    println!(
                        "Message is too long for the provided image file, use larger file"
                    );
                }
                errors::IMAGE_TOO_BIG => {
                    let lim = err
                        .raw()
                        .and_then(|j| j.get("limits"))
                        .and_then(|l| l.get("maxFileSize"))
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    println!("Image file is too big, current limit is set to {lim}");
                }
                errors::LIMIT_MESSAGE => {
                    let lim = err
                        .raw()
                        .and_then(|j| j.get("limits"))
                        .and_then(|l| l.get("maxMessageLen"))
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    println!("Message is too long, current limit is set to {lim}");
                }
                errors::LIMIT_PASSWORD => {
                    let lim = err
                        .raw()
                        .and_then(|j| j.get("limits"))
                        .and_then(|l| l.get("maxPasswordLen"))
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    println!("Password is too long, current limit is set to {lim}");
                }
                errors::INVALID_PASSWORD => {
                    println!("Invalid password");
                }
                _ => {
                    println!(
                        "An error occurred: {}",
                        err.error_message()
                    );
                }
            }
        }
    }
}
