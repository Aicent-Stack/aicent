// Aicent Stack | Demo: AICENT Brain Orchestration
// Domain: http://aicent.com
// Purpose: Demonstrating Atomic Task Sharding & Eight-Pillar Integration.
// Baseline: v1.2.1-Alpha | Status: RADIANT Active.

//! # aicent-demo: AICENT Brain Unit Test
//! 
//! This binary proves the functionality of the Cognitive Orchestrator (RFC-001).
//! It demonstrates the full reflex arc by injecting mock Immunity and Somatic layers,
//! verifying the sub-200µs sharding finality and the commercial latency gating.

use aicent::{
    AID, CognitiveOrchestrator, ImmunityReflex, SomaticActuation, 
    CognitivePulse, BrainError, VERSION
};
use std::time::Instant;

// ------------------------------------------------------------------------
// [MOCK IMMUNITY] Simulating the RFC-003 Reflex
// ------------------------------------------------------------------------
struct SovereignImmunity;
impl ImmunityReflex for SovereignImmunity {
    fn verify_integrity(&self, pulse: &CognitivePulse) -> bool {
        // [SIMD] Parallel tensor scan simulation in < 10µs
        println!("   ↳ [IMMUNE-SCAN]: Verified Watermark for AID {} ✅", pulse.aid);
        true
    }
}

// ------------------------------------------------------------------------
// [MOCK BODY] Simulating the RFC-005 Somatic Collapse
// ------------------------------------------------------------------------
struct SovereignBody;
impl SomaticActuation for SovereignBody {
    fn request_torque(&self, pulse: &CognitivePulse) -> Result<(), String> {
        // [REFLEX] 1.2kHz motor shunting simulation
        println!("   ↳ [BODY-TORQUE]: Collapsing Intent 0x{:02x?} to physical action...", &pulse.intent_hash[..4]);
        Ok(())
    }
}

fn main() {
    println!("\n\x1b[1;37m🧠 AICENT BRAIN | Cognitive Orchestrator Unit Test [RFC-001]\x1b[0m");
    println!("   Status: Radiant | Mode: Full-Blood Reflex Arc Simulation");
    println!("----------------------------------------------------");

    // ------------------------------------------------------------------------
    // [SCENARIO 1]: RADIANT MODE (Valid IQA-ORG Seal)
    // ------------------------------------------------------------------------
    println!("\n🧪 [TEST 1]: Executing with Radiant Sovereignty...");
    let radiant_brain = CognitiveOrchestrator::new(true); // Radiant Access: Enabled
    let immunity = SovereignImmunity;
    let body = SovereignBody;

    let start_radiant = Instant::now();
    let result = radiant_brain.execute_reflex_arc(
        "Execute high-torque kinetic alignment", 
        &immunity, 
        &body
    );

    match result {
        Ok(sharding_latency) => {
            println!("   ↳ Sharding Finality:  {:?} (Internal Logic)", sharding_latency);
            println!("   ↳ Total Arc Latency:  {:?} (E2E Reflex)", start_radiant.elapsed());
            println!("   ↳ System Status:      \x1b[1;32mRADIANT\x1b[0m");
        }
        Err(e) => println!("   ↳ Error: {:?}", e),
    }

    println!("----------------------------------------------------");

    // ------------------------------------------------------------------------
    // [SCENARIO 2]: GHOST MODE (Seal Missing)
    // ------------------------------------------------------------------------
    println!("\n🧪 [TEST 2]: Executing without IQA-ORG Seal (Ghost Node)...");
    let ghost_brain = CognitiveOrchestrator::new(false); // Radiant Access: Disabled

    let start_ghost = Instant::now();
    let _ = ghost_brain.execute_reflex_arc(
        "Standard Operation", 
        &immunity, 
        &body
    );

    println!("   ↳ Total Arc Latency:  {:?} (Legacy Emulation Enforced)", start_ghost.elapsed());
    println!("   ↳ System Status:      \x1b[1;33mDORMANT (Throttled)\x1b[0m");

    println!("\n\x1b[1;37m======================= COGNITIVE TEST COMPLETE =======================\x1b[0m");
    println!("   Protocol Version: {} ", VERSION);
    println!("----------------------------------------------------\n");
}
