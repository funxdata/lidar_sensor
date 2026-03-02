use anyhow::Result;

// 配置结构体，可以从配置文件加载
#[derive(Debug)]
pub struct LidarConfig {
    pub port: String,
    pub baud_rate: u32,
    pub timeout_ms: u64,
}

impl LidarConfig {
    pub fn load() -> Result<Self> {
        // 这里可以加载配置文件（如 config.json），现在用默认配置
        Ok(LidarConfig {
            port: "/dev/ttyUSB0".to_string(),
            baud_rate: 230400,
            timeout_ms: 100,
        })
    }
}