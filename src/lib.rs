// Aicent Stack | AICENT (The Brain)
// Domain: http://aicent.com
// Purpose: Cognitive Orchestration & Sovereign Identity (AID).
// Specification: RFC-001 Standard (Active).
// License: Apache-2.0 via Aicent.com Organization.
//! # RFC-001: Aicent Brain Orchestration Protocol
//! 
//! The `aicent` crate serves as the master decision-making hub for the Aicent Stack.
//! It governs the sovereign lifecycle of AI agents through cognitive decomposition 
//! and evolutionary feedback loops across a six-domain biological architecture.
//! 
//! ### Core Governance:
//! - **Sovereign AID Management**: Cryptographically bound identity resolution.
//! - **Instruction Sharding**: Decomposing high-level intent into atomic primitives.
//! - **Evolutionary Scheduling**: Real-time optimization of compute paths via GTIOT feedback.
//! - **Hive Convergence**: Synchronizing individual cognition with the Aicent.net Grid (RFC-006).

#![deny(missing_docs)]
#![deny(unsafe_code)]

/// [RFC-001] Internal Brain Logic
pub mod brain;

// Re-exporting core types for seamless workspace integration
pub use crate::brain::{
    Brain, 
    SovereignAID, 
    TaskPrimitive, 
    CognitivePulse,
    EvolutionaryScheduler
};

/// [RFC-001] Cognitive Error Set
/// Defines the failure modes of the orchestration layer.
#[derive(Debug, Clone, PartialEq)]
pub enum BrainError {
    /// Failed to resolve the AID fingerprint via RPKI
    IdentityResolutionFailed,
    /// Intent was too high-entropy to shard into primitives
    DecompositionTimeout,
    /// Feedback loop from GTIOT indicated physical instability
    HomeostasisBreach,
    /// Hive synchronization rejected local state hash
    HiveAlignmentError,
}

/// [RFC-001] Orchestration Traits
/// Defines the mandatory behavior of a sovereign cognitive entity.
pub trait CognitiveOrchestrator {
    /// Ingests raw symbolic intent and produces a verifiable Cognitive Pulse.
    fn orchestrate(&self, intent: &str) -> Result<CognitivePulse, BrainError>;
    
    /// Calibrates the brain's scheduler based on ZCMK clearing metrics.
    fn calibrate_metabolism(&mut self, cost_index: f32);

    /// Aligns the internal state with the global Aicent.net Hive (RFC-006).
    fn align_with_hive(&mut self, hive_state: [u8; 32]) -> Result<(), BrainError>;
}

/// [Standard v1.0] Integrated Domain Coordination
/// This constant verifies that the crate adheres to the unified specification suite:
/// - RFC-001 (Brain) | RFC-002 (Nerves) | RFC-003 (Immunity)
/// - RFC-004 (Blood) | RFC-005 (Body)   | RFC-006 (Hive)
pub const PROTOCOL_VERSION: &str = "1.0.0-standard-active";

/// High-fidelity telemetry marker for cognitive cycles.
pub fn log_cognitive_event(msg: &str) {
    println!("\x1b[1;37m[AICENT-BRAIN]\x1b[0m 🧠 {}", msg);
}
