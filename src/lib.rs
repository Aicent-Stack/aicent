// Aicent Stack | AICENT (The Brain)
// Domain: http://aicent.com
// Purpose: Cognitive Orchestration & Sovereign Identity (AID).
// Specification: RFC-001 Standard (Active).
// License: Apache-2.0 via Aicent.com Organization.
//! # RFC-001: Aicent Brain Orchestration Protocol
//! 
//! The `aicent` crate serves as the master decision-making hub for the Aicent Stack.
//! It governs the sovereign lifecycle of AI agents through cognitive decomposition 
//! and evolutionary feedback loops.
//! 
//! ### Core Governance:
//! - **Sovereign AID Management**: Cryptographically bound identity resolution.
//! - **Instruction Sharding**: Decomposing high-level intent into atomic primitives.
//! - **Evolutionary Scheduling**: Real-time optimization of compute paths via GTIOT feedback.
//! - **Hive Convergence**: Synchronizing individual cognition with the Aicent.net Grid (RFC-006).

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod brain;

// Re-exporting core types for seamless workspace integration
pub use crate::brain::{
    Brain, 
    SovereignAID, 
    TaskPrimitive, 
    CognitivePulse,
    EvolutionaryScheduler
};

/// [RFC-001] Orchestration Traits
/// Defines the behavior of a sovereign cognitive entity.
pub trait CognitiveOrchestrator {
    /// Ingests raw intent and produces a verifiable Cognitive Pulse.
    fn orchestrate(&self, intent: &str) -> CognitivePulse;
    
    /// Aligns the internal state with the global Hive (RFC-006).
    fn align_with_hive(&mut self, hive_state: [u8; 32]) -> bool;
}

/// [Standard v1.0] Integrated Domain Coordination
/// This crate operates as the logic layer for:
/// - Nerves (RTTP / RFC-002)
/// - Immunity (RPKI / RFC-003)
/// - Blood (ZCMK / RFC-004)
/// - Body (GTIOT / RFC-005)
/// - Hive (AICENT-NET / RFC-006)
pub const PROTOCOL_VERSION: &str = "0.1.0-standard";
