use serde::Deserialize;
use std::fs;
use std::path::Path;

/* =========================
   ROI 原始数据结构
========================= */
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct RoiConfig {
    pub angle_min: f32,
    pub angle_max: f32,
    pub range_min: f32,
    pub range_max: f32,
}

/* =========================
   ROI 管理器
========================= */
#[derive(Clone, Debug, Default)]
pub struct RoiManager {
    pub config: Option<RoiConfig>,
}

impl RoiManager {
    /* =========================
       加载 ROI
       - 文件不存在 → None
       - 空文件 → None
       - JSON错误 → None
    ========================== */
    pub fn load(path: &str) -> Self {
        if !Path::new(path).exists() {
            return Self { config: None };
        }

        let text = fs::read_to_string(path).unwrap_or_default();
        if text.trim().is_empty() {
            return Self { config: None };
        }

        let cfg: Result<RoiConfig, _> = serde_json::from_str(&text);

        match cfg {
            Ok(v) => Self { config: Some(v) },
            Err(_) => Self { config: None },
        }
    }

    /* =========================
       是否启用 ROI
    ========================== */
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.config.is_none()
    }

    /* =========================
       点是否在 ROI 内
    ========================== */
    #[inline(always)]
    pub fn contains(&self, angle: f32, range: f32) -> bool {
        match &self.config {
            Some(r) => {
                angle >= r.angle_min
                    && angle <= r.angle_max
                    && range >= r.range_min
                    && range <= r.range_max
            }
            None => true, // ⭐ 关键：没有 ROI → 全通过
        }
    }
}
