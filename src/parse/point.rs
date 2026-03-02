pub fn parse_frame(frame: &[u8]) {
    if frame.len() < 8 {
        return;
    }

    let mut start =
        u16::from_le_bytes([frame[4], frame[5]]) as f32 / 100.0;
    let mut end =
        u16::from_le_bytes([frame[6], frame[7]]) as f32 / 100.0;

    start %= 360.0;
    end %= 360.0;

    if end < start {
        end += 360.0;
    }

    // 点数量
    let point_count = (frame.len() - 8) / 2;

    if point_count == 0 {
        return;
    }

    let angle_step = if point_count > 1 {
        (end - start) / (point_count as f32 - 1.0)
    } else {
        0.0
    };

    for i in 0..point_count {
        let idx = 8 + i * 2;

        if idx + 1 >= frame.len() {
            break;
        }

        let distance =
            u16::from_le_bytes([frame[idx], frame[idx + 1]]);

        let mut angle = start + angle_step * i as f32;
        angle %= 360.0;

        if distance > 0 {
            println!(
                "angle: {:6.2}°, distance: {:5} mm",
                angle,
                distance
            );
        }
    }
}