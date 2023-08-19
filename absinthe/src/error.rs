use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Actor failed to start")]
    StartFailed,
    #[error("Actor failed to stop")]
    StopFailed,
    #[error("Actor failed to spawn")]
    SpawnFailed,
}