// Aicent Stack | AICENT (The Brain)
// Domain: http://aicent.com
// Purpose: Unit Demonstration of AID Identity & Task Sharding (RFC-001)
// License: Apache-2.0

use aicent::brain::{Brain, SovereignAID};

fn main() {
    println!("\x1b[1;37m🧠 AICENT BRAIN | Organ Function Test [RFC-001]\x1b[0m");
    println!("----------------------------------------------------");

    let mut brain = Brain::new();

    // 1. Resolve a Sovereign Identity (AID)
    let aid = SovereignAID {
        fingerprint: [0x88; 32],
        epoch: 1,
        reputation: 0.99,
    };
    println!("🛡️  AID Resolved: 0x882_Alpha (Reputation: {})", aid.reputation);

    // 2. High-level cognitive intent
    let intents = vec![
        "Stabilize Edge-882 vibration via active damping",
        "Coordinate swarm resonance across Aicent.net Hive",
        "Execute nanosecond resource auction via ZCMK",
    ];

    // 3. Demonstrate Task Sharding (Decomposition)
    for intent in intents {
        println!("\n🔮 Ingesting Intent: \"{}\"", intent);
        let primitives = brain.decompose_task(&aid, intent);
        
        for p in primitives {
            println!("   ↳ Sharded Primitive ID: 0x{:x}", p.primitive_id);
            println!("     Target Endpoint: rttp://{}", p.semantic_target);
        }
    }

    println!("\n✅ RFC-001 Cognitive Cycle Test Complete.");
    println!("   System status: HOMEOSTASIS.");
}
