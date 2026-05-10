use tokio::sync::mpsc;
use serde::Serialize;
use std::os::raw::c_void;
use std::time::Duration;

use crate::roi::RoiManager;

/* =========================
   点结构
========================= */
#[repr(C)]
#[derive(Clone, Copy, Debug, Serialize)]
pub struct LidarPoint {
    pub angle: f32,
    pub range: f32,
    pub intensity: f32,
}

#[derive(Clone, Serialize, Debug)]
pub struct ScanFrame {
    pub points: Vec<LidarPoint>,
}

/* =========================
   FFI
========================= */
extern "C" {
    pub fn ydlidar_create() -> *mut c_void;

    pub fn ydlidar_initialize(
        lidar: *mut c_void,
        port: *const i8,
        baudrate: i32,
    ) -> i32;

    pub fn ydlidar_turn_on(lidar: *mut c_void) -> i32;

    pub fn ydlidar_get_scan(
        lidar: *mut c_void,
        points: *mut LidarPoint,
        max_points: i32,
    ) -> i32;
}

/* =========================
   LiDAR 主线程
========================= */
pub fn spawn_lidar_thread(tx: mpsc::Sender<ScanFrame>) {
    std::thread::spawn(move || unsafe {

        let lidar = ydlidar_create();
        if lidar.is_null() {
            println!("❌ lidar create failed");
            return;
        }

        let port = std::ffi::CString::new("/dev/ttyUSB0").unwrap();

        let ret = ydlidar_initialize(lidar, port.as_ptr(), 230400);
        if ret != 0 {
            println!("  init warning: {}", ret);
        }

        ydlidar_turn_on(lidar);
        println!("🚀 LiDAR started");

        // =========================
        // ROI 加载（只加载一次）
        // =========================
        let path = "./dataset/area.json";
        let mut roi = RoiManager::load(path);

        println!("📄 ROI file: {}", path);
        match &roi.config {
            Some(c) => println!(
                "🎯 ROI ON angle:[{:.2},{:.2}] range:[{:.2},{:.2}]",
                c.angle_min, c.angle_max, c.range_min, c.range_max
            ),
            None => println!("🟢 ROI OFF → full scan mode"),
        }

        loop {
            let mut buf = vec![
                LidarPoint { angle: 0.0, range: 0.0, intensity: 0.0 };
                2048
            ];

            let n = ydlidar_get_scan(lidar, buf.as_mut_ptr(), 2048);

            if n <= 0 {
                std::thread::sleep(Duration::from_millis(5));
                continue;
            }

            buf.truncate(n as usize);

            // =========================
            // ROI 过滤（核心）
            // =========================
            let points = if roi.is_empty() {
                buf
            } else {
                let mut out = Vec::with_capacity(buf.len());

                for p in buf {
                    if roi.contains(p.angle, p.range) {
                        out.push(p);
                    }
                }

                out
            };

            let frame = ScanFrame { points };

            if tx.blocking_send(frame).is_err() {
                println!("channel closed");
                break;
            }

            std::thread::sleep(Duration::from_millis(10));
        }
    });
}

