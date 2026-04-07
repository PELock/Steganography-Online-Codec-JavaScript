/******************************************************************************
 *
 * Steganography Online Codec WebApi interface usage example.
 *
 * In this example, we will see how to extract a previously encrypted & hidden
 * secret message from an image file.
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

    let input_file_path = "output_file_with_hidden_secret_message.png";
    let password = "Pa$$word";

    match my_steganography_online_codec
        .decode(input_file_path, password)
        .await
    {
        Ok(result) => {
            let full = result
                .license
                .as_ref()
                .and_then(|l| l.activation_status)
                .unwrap_or(false);
            println!(
                "You are running in {} version",
                if full { "full" } else { "demo" }
            );

            let msg = result.message.as_deref().unwrap_or("");
            println!("Secret message is \"{msg}\"");

            if let Some(ref lic) = result.license {
                if let Some(c) = lic.usages_count {
                    println!("Remaining number of usage credits - {c}");
                }
            }
        }
        Err(err) => match err.code() {
            errors::INVALID_INPUT => {
                println!(
                    "Invalid input file {} or file doesn't exist",
                    input_file_path
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
                println!(
                    "Extracted message is too long, current limit is set to {lim}"
                );
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
                println!("An error occurred: {}", err.error_message());
            }
        },
    }
}
