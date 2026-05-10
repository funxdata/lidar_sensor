mod lidar;
mod socket;
mod roi;
use tokio::sync::mpsc;
use tokio::sync::broadcast;

use lidar::{spawn_lidar_thread, ScanFrame};
use socket::ws_server;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<ScanFrame>(32);
    let (btx, brx) = broadcast::channel::<ScanFrame>(32);
    spawn_lidar_thread(tx);

    tokio::spawn(async move {
        let mut rx = rx;
        while let Some(frame) = rx.recv().await {
            let _ = btx.send(frame);
        }
    });

    ws_server(brx).await;
}