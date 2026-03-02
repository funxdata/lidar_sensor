use serialport;
use std::time::Duration;
use std::io::{Read, Write};
use std::collections::VecDeque;
use crate::parse::point;
use anyhow::{Context, Result};
use crate::lidar::config::LidarConfig;

// lidar run
pub fn run_lidar(config: LidarConfig) -> Result<()> {
    let mut port = serialport::new(&config.port, config.baud_rate)
        .timeout(Duration::from_millis(config.timeout_ms))
        .open()
        .context("Failed to open serial port")?;

    println!("Connected to {}", config.port);

    // 启动扫描
    port.write_all(&[0xA5, 0x60])
        .context("Failed to send start scan command")?;

    let mut buffer = [0u8; 1024];
    let mut frame: VecDeque<u8> = VecDeque::new();

    // 循环读取数据
    loop {
        match port.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    continue; // 如果没有读取到数据，跳过
                }

                frame.extend(&buffer[..n]);

                // 解析数据
                process_frame(&mut frame)?;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                // 超时不做任何处理
            }
            Err(e) => {
                eprintln!("Serial error: {:?}", e);
                break; // 发生严重错误时退出
            }
        }
    }

    Ok(())
}

// 处理帧数据
fn process_frame(frame: &mut VecDeque<u8>) -> Result<()> {
    while frame.len() >= 4 {
        // 同步帧头 AA 55
        if frame[0] != 0xAA || frame[1] != 0x55 {
            // 如果不是帧头，移除一个字节继续检查
            frame.pop_front();
            continue;
        }

        let len = frame[3] as usize;
        let full_len = 4 + len;

        // 如果当前帧数据不完整，等待更多数据
        if frame.len() < full_len {
            break;
        }

        // 提取完整的数据包
        let packet: Vec<u8> = frame.drain(..full_len).collect();

        // 解析帧数据
        if packet.len() >= 8 {
            point::parse_frame(&packet);
        }
    }

    Ok(())
}