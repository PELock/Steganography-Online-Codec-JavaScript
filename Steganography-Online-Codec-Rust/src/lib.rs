//! Steganography Online Codec Web API client (Rust).
//!
//! See the product page: <https://www.pelock.com/products/steganography-online-codec>

use std::path::Path;

use base64::Engine;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

/// Error codes returned by the Steganography Online Codec API (same values as the JavaScript SDK).
pub mod errors {
    /// Cannot connect to the Web API interface (network error).
    pub const WEBAPI_CONNECTION: i32 = -1;
    /// Success.
    pub const SUCCESS: i32 = 0;
    /// Unknown error.
    pub const UNKNOWN: i32 = 1;
    /// Message is too long for the selected image file.
    pub const MESSAGE_TOO_LONG: i32 = 2;
    /// Image file is too big.
    pub const IMAGE_TOO_BIG: i32 = 3;
    /// Invalid input file or file does not exist.
    pub const INVALID_INPUT: i32 = 4;
    /// Image file format is not supported.
    pub const INVALID_IMAGE_FORMAT: i32 = 5;
    /// Image file is malformed.
    pub const IMAGE_MALFORMED: i32 = 6;
    /// Invalid password.
    pub const INVALID_PASSWORD: i32 = 7;
    /// Provided message is too long for the current license tier.
    pub const LIMIT_MESSAGE: i32 = 9;
    /// Provided password is too long for the current license tier.
    pub const LIMIT_PASSWORD: i32 = 10;
    /// Error while writing output file.
    pub const OUTPUT_FILE: i32 = 99;
    /// License key is invalid or expired.
    pub const INVALID_LICENSE: i32 = 100;
}

/// License information returned by the API (`license` object).
#[derive(Debug, Clone, Deserialize)]
pub struct LicenseInfo {
    #[serde(rename = "activationStatus")]
    pub activation_status: Option<bool>,
    #[serde(rename = "userName")]
    pub user_name: Option<String>,
    #[serde(rename = "type")]
    pub license_type: Option<i64>,
    #[serde(rename = "usagesTotal")]
    pub usages_total: Option<i64>,
    #[serde(rename = "usagesCount")]
    pub usages_count: Option<i64>,
}

/// Current limits returned by the API (`limits` object).
#[derive(Debug, Clone, Deserialize)]
pub struct LimitsInfo {
    #[serde(rename = "maxPasswordLen")]
    pub max_password_len: Option<i64>,
    #[serde(rename = "maxMessageLen")]
    pub max_message_len: Option<i64>,
    #[serde(rename = "maxFileSize")]
    pub max_file_size: Option<i64>,
}

/// Successful API response fields used by the SDK (after `error == SUCCESS`).
#[derive(Debug, Clone)]
pub struct CodecResult {
    pub license: Option<LicenseInfo>,
    pub limits: Option<LimitsInfo>,
    /// Decoded secret message (`decode` only).
    pub message: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum SteganographyError {
    #[error("{error_message}")]
    Api {
        code: i32,
        error_message: String,
        raw: Option<serde_json::Value>,
    },
}

impl SteganographyError {
    pub fn code(&self) -> i32 {
        match self {
            SteganographyError::Api { code, .. } => *code,
        }
    }

    pub fn error_message(&self) -> &str {
        match self {
            SteganographyError::Api { error_message, .. } => error_message,
        }
    }

    pub fn raw(&self) -> Option<&serde_json::Value> {
        match self {
            SteganographyError::Api { raw, .. } => raw.as_ref(),
        }
    }
}

/// Default Steganography Online Codec Web API endpoint.
pub const DEFAULT_API_URL: &str = "https://www.pelock.com/api/steganography-online-codec/v1";

/// Client for the Steganography Online Codec Web API.
pub struct SteganographyOnlineCodec {
    api_key: Option<String>,
    api_url: String,
    client: reqwest::Client,
}

impl SteganographyOnlineCodec {
    /// Create a new client. Pass `None` or empty string for demo mode (no activation key).
    pub fn new(api_key: Option<String>) -> Self {
        Self::with_url(api_key, DEFAULT_API_URL.to_string())
    }

    /// Create a client with a custom API base URL (mainly for testing).
    pub fn with_url(api_key: Option<String>, api_url: String) -> Self {
        let key = api_key.filter(|s| !s.is_empty());
        Self {
            api_key: key,
            api_url,
            client: reqwest::Client::new(),
        }
    }

    /// Login and retrieve license and limit information.
    pub async fn login(&self) -> Result<CodecResult, SteganographyError> {
        let form = self.build_form("login", |f| f);
        self.post_request_codec_result(form).await
    }

    /// Encrypt and hide a message in an image; writes a PNG to `output_image_path`.
    pub async fn encode(
        &self,
        input_image_path: impl AsRef<Path>,
        message_to_hide: &str,
        password: &str,
        output_image_path: impl AsRef<Path>,
    ) -> Result<CodecResult, SteganographyError> {
        let path = input_image_path.as_ref();
        let bytes = tokio::fs::read(path)
            .await
            .map_err(|e| SteganographyError::Api {
                code: errors::INVALID_INPUT,
                error_message: e.to_string(),
                raw: None,
            })?;
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("image.bin");
        self.encode_bytes(&bytes, filename, message_to_hide, password, output_image_path)
            .await
    }

    /// Same as [`encode`](Self::encode) but reads image bytes you already have in memory.
    pub async fn encode_bytes(
        &self,
        image: &[u8],
        filename_for_upload: &str,
        message_to_hide: &str,
        password: &str,
        output_image_path: impl AsRef<Path>,
    ) -> Result<CodecResult, SteganographyError> {
        if image.is_empty() {
            return Err(SteganographyError::Api {
                code: errors::INVALID_INPUT,
                error_message: "input_image is required".to_string(),
                raw: None,
            });
        }
        if message_to_hide.is_empty() {
            return Err(SteganographyError::Api {
                code: errors::INVALID_INPUT,
                error_message: "message_to_hide is required".to_string(),
                raw: None,
            });
        }
        if password.is_empty() {
            return Err(SteganographyError::Api {
                code: errors::INVALID_INPUT,
                error_message: "password is required".to_string(),
                raw: None,
            });
        }
        let out = output_image_path.as_ref();
        if out.as_os_str().is_empty() {
            return Err(SteganographyError::Api {
                code: errors::INVALID_INPUT,
                error_message: "output_image_path is required".to_string(),
                raw: None,
            });
        }

        let image_part = Part::bytes(image.to_vec())
            .file_name(filename_for_upload.to_string())
            .mime_str("application/octet-stream")
            .map_err(|e| SteganographyError::Api {
                code: errors::UNKNOWN,
                error_message: e.to_string(),
                raw: None,
            })?;

        let form = self.build_form("encode", |f| {
            f.text("message", message_to_hide.to_string())
                .text("password", password.to_string())
                .part("image", image_part)
        });

        let json = self.post_request_json(form).await?;
        let encoded_b64 = json
            .get("encodedImage")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SteganographyError::Api {
                code: errors::UNKNOWN,
                error_message: "Malformed API response: missing encodedImage".to_string(),
                raw: Some(json.clone()),
            })?;

        let decoded = base64::engine::general_purpose::STANDARD
            .decode(encoded_b64)
            .map_err(|e| SteganographyError::Api {
                code: errors::UNKNOWN,
                error_message: e.to_string(),
                raw: Some(json.clone()),
            })?;

        tokio::fs::write(output_image_path.as_ref(), &decoded)
            .await
            .map_err(|e| SteganographyError::Api {
                code: errors::OUTPUT_FILE,
                error_message: e.to_string(),
                raw: Some(json.clone()),
            })?;

        Self::json_to_codec_result(json)
    }

    /// Extract a hidden message from a PNG image.
    pub async fn decode(
        &self,
        input_image_path: impl AsRef<Path>,
        password: &str,
    ) -> Result<CodecResult, SteganographyError> {
        let path = input_image_path.as_ref();
        let bytes = tokio::fs::read(path)
            .await
            .map_err(|e| SteganographyError::Api {
                code: errors::INVALID_INPUT,
                error_message: e.to_string(),
                raw: None,
            })?;
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("image.png");
        self.decode_bytes(&bytes, filename, password).await
    }

    /// Same as [`decode`](Self::decode) but uses in-memory image bytes.
    pub async fn decode_bytes(
        &self,
        image: &[u8],
        filename_for_upload: &str,
        password: &str,
    ) -> Result<CodecResult, SteganographyError> {
        if image.is_empty() {
            return Err(SteganographyError::Api {
                code: errors::INVALID_INPUT,
                error_message: "input_image is required".to_string(),
                raw: None,
            });
        }
        if password.is_empty() {
            return Err(SteganographyError::Api {
                code: errors::INVALID_INPUT,
                error_message: "password is required".to_string(),
                raw: None,
            });
        }

        let image_part = Part::bytes(image.to_vec())
            .file_name(filename_for_upload.to_string())
            .mime_str("application/octet-stream")
            .map_err(|e| SteganographyError::Api {
                code: errors::UNKNOWN,
                error_message: e.to_string(),
                raw: None,
            })?;

        let form = self.build_form("decode", |f| {
            f.text("password", password.to_string())
                .part("image", image_part)
        });

        self.post_request_codec_result(form).await
    }

    /// Convert a byte count to a human-readable string (e.g. `"1.23 MB"`), matching the JavaScript SDK.
    pub fn convert_size(size_bytes: i64) -> String {
        Self::convert_size_f64(size_bytes as f64)
    }

    fn convert_size_f64(size_bytes: f64) -> String {
        if !size_bytes.is_finite() || size_bytes < 0.0 {
            return "0 bytes".to_string();
        }
        if size_bytes == 0.0 {
            return "0 bytes".to_string();
        }

        const NAMES: &[&str] = &[
            "bytes", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB",
        ];
        let i = ((size_bytes.ln() / 1024_f64.ln()).floor() as usize).min(NAMES.len() - 1);
        let p = 1024_f64.powi(i as i32);
        let s = ((size_bytes / p) * 100.0).round() / 100.0;
        format!("{} {}", s, NAMES[i])
    }

    fn key_field(&self) -> String {
        self.api_key.clone().unwrap_or_default()
    }

    fn build_form(&self, command: &'static str, extend: impl FnOnce(Form) -> Form) -> Form {
        let base = Form::new()
            .text("key", self.key_field())
            .text("command", command.to_string());
        extend(base)
    }

    async fn post_request_json(&self, form: Form) -> Result<serde_json::Value, SteganographyError> {
        let response = self
            .client
            .post(&self.api_url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| SteganographyError::Api {
                code: errors::WEBAPI_CONNECTION,
                error_message: e.to_string(),
                raw: None,
            })?;

        if !response.status().is_success() {
            return Err(SteganographyError::Api {
                code: errors::WEBAPI_CONNECTION,
                error_message: format!("HTTP {}", response.status()),
                raw: None,
            });
        }

        let json: serde_json::Value = response.json().await.map_err(|e| {
            SteganographyError::Api {
                code: errors::WEBAPI_CONNECTION,
                error_message: e.to_string(),
                raw: None,
            }
        })?;

        let err_code = json
            .get("error")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let Some(code) = err_code else {
            return Err(SteganographyError::Api {
                code: errors::UNKNOWN,
                error_message: "Malformed API response: missing error field".to_string(),
                raw: Some(json),
            });
        };

        if code == errors::SUCCESS {
            return Ok(json);
        }

        let msg = json
            .get("error_message")
            .and_then(|v| v.as_str())
            .unwrap_or("API error")
            .to_string();

        Err(SteganographyError::Api {
            code,
            error_message: msg,
            raw: Some(json),
        })
    }

    async fn post_request_codec_result(
        &self,
        form: Form,
    ) -> Result<CodecResult, SteganographyError> {
        let json = self.post_request_json(form).await?;
        Self::json_to_codec_result(json)
    }

    fn json_to_codec_result(json: serde_json::Value) -> Result<CodecResult, SteganographyError> {
        let license = json
            .get("license")
            .cloned()
            .and_then(|v| serde_json::from_value::<LicenseInfo>(v).ok());
        let limits = json
            .get("limits")
            .cloned()
            .and_then(|v| serde_json::from_value::<LimitsInfo>(v).ok());
        let message = json
            .get("message")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(CodecResult {
            license,
            limits,
            message,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_size_matches_js_style() {
        assert_eq!(SteganographyOnlineCodec::convert_size(0), "0 bytes");
        assert_eq!(SteganographyOnlineCodec::convert_size(500), "500 bytes");
    }
}
