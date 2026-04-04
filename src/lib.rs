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
//! - **Sovereign AID Management**: Cryptographically bound 128-bit atomic identity resolution.
//! - **Instruction Sharding**: Decomposing high-level symbolic intent into atomic motor primitives.
//! - **Evolutionary Scheduling**: Real-time optimization of compute paths via GTIOT proprioception.
//! - **Hive Convergence**: Synchronizing individual cognition with the Aicent.net operational grid.

#![deny(missing_docs)]
// SAFETY: The cognitive layer must remain absolutely deterministic and memory-safe.
// Hardware-level unsafe primitives are restricted to the Nerves (RTTP) and Body (GTIOT).
#![deny(unsafe_code)]

/// [RFC-001] Internal Brain Logic.
/// Contains the orchestration engine and the Evolutionary Scheduler.
pub mod brain;

// Re-exporting core types for seamless workspace integration and API clarity.
pub use crate::brain::{
    Brain, CognitivePulse, EvolutionaryScheduler, IdentityState, SovereignAID, TaskPrimitive,
};

/// [RFC-001] Cognitive Error Set.
/// Defines the critical failure modes within the orchestration layer.
#[derive(Debug, Clone, PartialEq)]
pub enum BrainError {
    /// Failed to resolve the AID fingerprint via the RPKI Merkle-DAG (RFC-003).
    IdentityResolutionFailed,
    /// High-level intent was too complex (high entropy) to shard in <200µs.
    DecompositionTimeout,
    /// Proprioceptive feedback from GTIOT (RFC-005) indicated physical instability.
    HomeostasisBreach,
    /// Hive synchronization (RFC-006) rejected the local state manifold.
    HiveAlignmentError,
}

/// [RFC-001] Orchestration Interface.
/// Defines the mandatory behavior of a sovereign cognitive entity.
/// Any third-party AI agent integrating with the Aicent Stack must implement this trait.
pub trait CognitiveOrchestrator {
    /// [RFC-001] Ingests raw symbolic intent and produces a verifiable Cognitive Pulse.
    /// This process constitutes the "Instruction Sharding" phase.
    fn orchestrate(&self, intent: &str) -> Result<CognitivePulse, BrainError>;

    /// [RFC-004] Calibrates the brain's evolutionary scheduler based on ZCMK 
    /// metabolic clearing metrics, ensuring economic homeostasis.
    fn calibrate_metabolism(&mut self, cost_index: f32);

    /// [RFC-006] Aligns the internal state manifold with the global Aicent.net Hive.
    /// This establishes "Collective Consciousness" across the planetary grid.
    fn align_with_hive(&mut self, hive_state: [u8; 32]) -> Result<(), BrainError>;
}

// --- Protocol Constants & Metadate ---

/// [Standard v1.0] Integrated Domain Coordination
/// Verifies that the crate adheres to the unified specification suite:
/// - RFC-001 (Brain) | RFC-002 (Nerves) | RFC-003 (Immunity)
/// - RFC-004 (Blood) | RFC-005 (Body)   | RFC-006 (Hive)
pub const PROTOCOL_VERSION: &str = "1.0.0-standard-active";

/// High-fidelity telemetry marker for cognitive cycles.
/// Utilizes ANSI color codes for biological system tracking.
pub fn log_cognitive_event(msg: &str) {
    println!("\x1b[1;37m[AICENT-BRAIN]\x1b[0m 🧠 {}", msg);
}
