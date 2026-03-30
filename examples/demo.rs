// Aicent Stack | AICENT (The Brain)
// Domain: https://aicent.com
// Purpose: AID Identity + Task Primitive Decomposition Demonstration
// Specification: RFC-001 Draft. 
// License: Apache-2.0 via Aicent.com Organization.
//! # RFC-001 Demo: Brain Orchestration Logic

use aicent::brain::Brain;

fn main() {
    println!("🧠 Aicent Brain - The Decision Center of Sovereign Lifeform");
    println!("====================================================");

    let mut brain = Brain::new();
    
    let tasks = vec![
        "Optimize 882 edge node vibration anomaly",
        "Coordinate RTTP pulse with RPKI verification",
        "Allocate ZCMK compute credits for GTIOT action",
    ];

    for task in tasks {
        let result = brain.decompose_task(task);
        println!("📋 Task: {}", task);
        println!("   → {}", result);
        println!();
    }

    println!("✅ Aicent Brain successfully decomposed 3 tasks.");
    println!("   Sovereign decision-making loop initialized.");
}
