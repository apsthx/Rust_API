use axum::{
    body::Bytes,
    http::StatusCode,
};
use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client as S3Client, primitives::ByteStream};
use image::ImageFormat;
use std::env;
use std::path::Path;
use anyhow::{Result, Context};
use tokio::fs;
use uuid::Uuid;

/// Allowed file types for upload
const ALLOWED_IMAGE_TYPES: &[&str] = &["image/jpeg", "image/jpg", "image/png", "image/gif"];
const ALLOWED_EXCEL_TYPES: &[&str] = &[
    "application/vnd.ms-excel",
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
];

/// File upload configuration
pub struct UploadConfig {
    pub max_size: usize,
    pub allowed_types: Vec<String>,
    pub upload_dir: String,
}

impl Default for UploadConfig {
    fn default() -> Self {
        Self {
            max_size: 10 * 1024 * 1024, // 10MB
            allowed_types: ALLOWED_IMAGE_TYPES.iter().map(|s| s.to_string()).collect(),
            upload_dir: "uploads/images".to_string(),
        }
    }
}

/// Upload result structure
#[derive(Debug)]
pub struct UploadResult {
    pub filename: String,
    pub path: String,
    pub url: String,
    pub size: usize,
}

/// Upload file to local filesystem with image resizing
/// Equivalent to Go's UploadFile function in middlewares/uploadfile.go
pub async fn upload_file(
    file_data: Bytes,
    content_type: &str,
    original_filename: &str,
    resize_width: Option<u32>,
) -> Result<UploadResult, (StatusCode, String)> {
    // Validate file type
    if !ALLOWED_IMAGE_TYPES.contains(&content_type) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Invalid file type: {}", content_type),
        ));
    }

    // Generate unique filename
    let extension = Path::new(original_filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg");

    let filename = format!("{}_{}.{}",
        Uuid::new_v4(),
        chrono::Utc::now().timestamp(),
        extension
    );

    // Create upload directory if not exists
    let upload_dir = env::var("UPLOAD_DIR").unwrap_or_else(|_| "uploads/images".to_string());
    fs::create_dir_all(&upload_dir)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let file_path = format!("{}/{}", upload_dir, filename);

    // Process and resize image if needed
    let processed_data = if let Some(width) = resize_width {
        resize_image(&file_data, width)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    } else {
        file_data.to_vec()
    };

    // Save file
    fs::write(&file_path, &processed_data)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8002".to_string());
    let url = format!("{}/{}", base_url, file_path);

    Ok(UploadResult {
        filename: filename.clone(),
        path: file_path,
        url,
        size: processed_data.len(),
    })
}

/// Upload file to AWS S3
/// Equivalent to Go's UploadS3 function
pub async fn upload_s3(
    file_data: Bytes,
    content_type: &str,
    original_filename: &str,
    folder: Option<&str>,
) -> Result<UploadResult, (StatusCode, String)> {
    // Validate file type
    if !ALLOWED_IMAGE_TYPES.contains(&content_type) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Invalid file type: {}", content_type),
        ));
    }

    // Generate unique filename
    let extension = Path::new(original_filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg");

    let filename = format!("{}_{}.{}",
        Uuid::new_v4(),
        chrono::Utc::now().timestamp(),
        extension
    );

    let s3_key = if let Some(f) = folder {
        format!("{}/{}", f, filename)
    } else {
        filename.clone()
    };

    // Initialize S3 client
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(env::var("AWS_REGION").unwrap_or_else(|_| "ap-southeast-1".to_string()))
        .load()
        .await;

    let s3_client = S3Client::new(&config);
    let bucket = env::var("AWS_BUCKET")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "AWS_BUCKET not set".to_string()))?;

    // Upload to S3
    s3_client
        .put_object()
        .bucket(&bucket)
        .key(&s3_key)
        .body(ByteStream::from(file_data.to_vec()))
        .content_type(content_type)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("S3 upload failed: {}", e)))?;

    let url = format!("https://{}.s3.amazonaws.com/{}", bucket, s3_key);

    Ok(UploadResult {
        filename: filename.clone(),
        path: s3_key,
        url,
        size: file_data.len(),
    })
}

/// Upload Excel file
/// Equivalent to Go's UploadExcel function
pub async fn upload_excel(
    file_data: Bytes,
    content_type: &str,
    original_filename: &str,
) -> Result<UploadResult, (StatusCode, String)> {
    // Validate file type
    if !ALLOWED_EXCEL_TYPES.contains(&content_type) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Invalid Excel file type: {}", content_type),
        ));
    }

    // Generate unique filename
    let extension = Path::new(original_filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("xlsx");

    let filename = format!("{}_{}.{}",
        Uuid::new_v4(),
        chrono::Utc::now().timestamp(),
        extension
    );

    // Create upload directory
    let upload_dir = env::var("EXCEL_UPLOAD_DIR")
        .unwrap_or_else(|_| "uploads/excels".to_string());
    fs::create_dir_all(&upload_dir)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let file_path = format!("{}/{}", upload_dir, filename);

    // Save file
    fs::write(&file_path, &file_data)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8002".to_string());
    let url = format!("{}/{}", base_url, file_path);

    Ok(UploadResult {
        filename: filename.clone(),
        path: file_path,
        url,
        size: file_data.len(),
    })
}

/// Resize image to specified width (maintains aspect ratio)
/// Equivalent to image processing in Go's UploadFile
fn resize_image(data: &[u8], width: u32) -> Result<Vec<u8>> {
    let img = image::load_from_memory(data)
        .context("Failed to load image")?;

    let resized = img.resize(width, u32::MAX, image::imageops::FilterType::Lanczos3);

    let mut output = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut output);

    resized.write_to(&mut cursor, ImageFormat::Jpeg)
        .context("Failed to encode resized image")?;

    Ok(output)
}

/// Check if file extension is allowed
pub fn check_extension(filename: &str, allowed_extensions: &[&str]) -> bool {
    Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .map(|ext| allowed_extensions.contains(&ext))
        .unwrap_or(false)
}

/// Validate file type
pub fn allow_file_type(content_type: &str, allowed_types: &[&str]) -> bool {
    allowed_types.contains(&content_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_extension() {
        assert!(check_extension("test.jpg", &["jpg", "png"]));
        assert!(check_extension("test.png", &["jpg", "png"]));
        assert!(!check_extension("test.txt", &["jpg", "png"]));
    }

    #[test]
    fn test_allow_file_type() {
        assert!(allow_file_type("image/jpeg", ALLOWED_IMAGE_TYPES));
        assert!(allow_file_type("image/png", ALLOWED_IMAGE_TYPES));
        assert!(!allow_file_type("text/plain", ALLOWED_IMAGE_TYPES));
    }
}
