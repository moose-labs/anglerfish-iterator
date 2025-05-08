use serde::{Deserialize, Serialize};
use sui_sdk::types::id::UID;

/// Rust representation of `anglerfish::phase::Phase`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Phase {
    Uninitialized,
    LiquidityProviding,
    Ticketing,
    Drawing,
    Distributing,
    Settling,
}

/// Rust representation of `anglerfish::phase::PhaseInfo`.
#[derive(Debug, Serialize, Deserialize)]
pub struct PhaseInfo {
    pub id: UID,
    pub current_round_number: u64,
    pub current_phase: Phase,
    pub current_phase_at: u64,
    pub durations: PhaseDurations,
    pub last_drawing_timestamp_ms: u64,
}

/// Rust representation of `anglerfish::phase::PhaseDurations`.
#[derive(Debug, Serialize, Deserialize)]
pub struct PhaseDurations {
    pub liquidity_providing_duration: u64,
    pub ticketing_duration: u64,
}
