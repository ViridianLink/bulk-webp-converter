use webp::WebPConfig;

pub fn config(quality: f32) -> WebPConfig {
    let mut webp_config = WebPConfig::new().expect("Failed to create WebPConfig");
    webp_config.quality = quality;
    webp_config.method = 6;

    if quality == 100.0 {
        webp_config.lossless = 1;
    }

    webp_config
}
