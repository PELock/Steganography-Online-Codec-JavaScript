/******************************************************************************
 *
 * Steganography Online Codec WebApi interface usage example.
 *
 * In this example we will verify our activation key status.
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

use steganography_online_codec::SteganographyOnlineCodec;

#[tokio::main]
async fn main() {
    let my_steganography_online_codec =
        SteganographyOnlineCodec::new(Some("YOUR-WEB-API-KEY".to_string()));

    match my_steganography_online_codec.login().await {
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

            if let Some(ref lic) = result.license {
                if lic.activation_status.unwrap_or(false) {
                    if let Some(ref name) = lic.user_name {
                        println!("Registered for - {name}");
                    }
                    let license_type = if lic.license_type == Some(0) {
                        "personal"
                    } else {
                        "company"
                    };
                    println!("License type - {license_type}");
                    if let Some(t) = lic.usages_total {
                        println!("Total number of purchased usage credits - {t}");
                    }
                    if let Some(c) = lic.usages_count {
                        println!("Remaining number of usage credits - {c}");
                    }
                }
            }

            if let Some(ref limits) = result.limits {
                if let Some(m) = limits.max_password_len {
                    println!("Max. password length - {m}");
                }
                if let Some(ml) = limits.max_message_len {
                    let msg_len = if ml == -1 {
                        "unlimited".to_string()
                    } else {
                        ml.to_string()
                    };
                    println!("Max. message length - {msg_len}");
                }
                if let Some(fs) = limits.max_file_size {
                    println!(
                        "Max. input image file size - {}",
                        SteganographyOnlineCodec::convert_size(fs)
                    );
                }
            }
        }
        Err(err) => {
            eprintln!("Login failed: {}", err.error_message());
        }
    }
}
