use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PaddleConfig {
    pub width: f32,
    pub height: f32,
    pub offset: f32,
    pub speed: f32,
}

impl Default for PaddleConfig {
    fn default() -> Self {
        PaddleConfig {
            width: 100f32,
            height: 10f32,
            offset: 10f32,
            speed: 6f32,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BallConfig {
    pub radius: f32,
    pub speed: f32,
}

impl Default for BallConfig {
    fn default() -> Self {
        BallConfig {
            radius: 5f32,
            speed: 200f32,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BlockConfig {
    pub width: f32,
    pub height: f32,
}

impl Default for BlockConfig {
    fn default() -> Self {
        BlockConfig {
            width: 50f32,
            height: 13f32,
        }
    }
}
