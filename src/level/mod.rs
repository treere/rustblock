pub use self::state::Level;

pub const SCREEN_WIDTH: f32 = 640.0;
pub const SCREEN_HEIGHT: f32 = 480.0;
pub const BALL_RADIUS: f32 = 5.;
pub const PADDLE_WIDTH: f32 = 100.;
pub const PADDLE_HEIGHT: f32 = 10.;
pub const PADDLE_OFFSET: f32 = 10.;
pub const BALL_SPEED: f32 = 200.;
pub const PADDLE_SPEED: f32 = 6.;
pub const BLOCK_WIDTH: f32 = 50.;
pub const BLOCK_HEIGHT: f32 = 13.;

mod util;
mod state;
mod component;
pub mod system;
