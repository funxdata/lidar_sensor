use lidar_sensor::lidar;
use lidar_sensor::lidar::config::LidarConfig;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    // 加载配置
    let config = LidarConfig::load().context("Failed to load config")?;
    
    // 启动雷达扫描
    println!("Lidar anchor scan starting...");

    if let Err(e) = lidar::run::run_lidar(config) {
        // 如果发生错误，打印详细错误信息并退出
        eprintln!("Error occurred during lidar scan: {:?}", e);
        return Err(e);
    }

    Ok(())
}