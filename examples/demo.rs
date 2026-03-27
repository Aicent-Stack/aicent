//! Aicent Brain Demo
//! AID Identity + Task Primitives Breakdown Demonstration

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
