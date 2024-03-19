use crate::QUALITY;
use webp::WebPConfig;

pub fn config() -> WebPConfig {
    let mut webp_config = WebPConfig::new().expect("Failed to create WebPConfig");
    webp_config.quality = QUALITY;
    webp_config.method = 6;

    if QUALITY == 100.0 {
        webp_config.lossless = 1;
    }

    webp_config
}
