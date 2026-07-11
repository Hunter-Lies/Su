use base64::Engine;
use image::Luma;
use qrcode::QrCode;

pub fn encode_qr_png(text: &str, size: u32) -> Result<String, String> {
    let code = QrCode::new(text).map_err(|e| format!("QR: {}", e))?;
    let img = code.render::<Luma<u8>>().min_dimensions(size, size).max_dimensions(size, size).build();
    let mut buf = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)
        .map_err(|e| format!("PNG: {}", e))?;
    Ok(format!("data:image/png;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(&buf)))
}
