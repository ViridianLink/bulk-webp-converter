use libwebp_sys::{WebPConfig, WebPImageHint};

pub fn config(quality: f32) -> WebPConfig {
    let mut webp_config = WebPConfig::new().unwrap();
    webp_config.quality = quality;
    webp_config.method = 6;
    webp_config.image_hint = WebPImageHint::WEBP_HINT_PICTURE;

    if quality == 100.0 {
        webp_config.lossless = 1;
    } else {
        webp_config.near_lossless = 60;
    }

    webp_config
}
