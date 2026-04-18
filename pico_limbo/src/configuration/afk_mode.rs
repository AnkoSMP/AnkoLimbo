use serde::{Deserialize, Serialize};

/// Configuration for AFK-mode behavior when running as a Limbo server.
#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AfkModeConfig {
    /// Enable AFK-mode automatic return.
    pub enabled: bool,

    /// Minimum movement duration (milliseconds) required to consider the player "moved"
    /// before returning them. Default: 500
    pub move_threshold_ms: u64,

    /// Minimum distance (Euclidean on XZ plane) required to count as movement.
    /// Default: 0.01
    pub move_distance: f64,

    /// Host to transfer the player back to when movement is detected (used instead of
    /// saving the origin on movement). If empty, no transfer is performed.
    pub return_host: String,

    /// Port to transfer the player back to.
    pub return_port: i32,
}

impl Default for AfkModeConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            move_threshold_ms: 500,
            move_distance: 0.01,
            return_host: String::new(),
            return_port: 25565,
        }
    }
}
