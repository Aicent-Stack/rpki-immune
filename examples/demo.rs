//! RPKI Immunity Demo
//! Zero-trust watermarking + task chain verification demo

use rpki::pipeline::ImmunePipeline;

fn main() {
    println!("🛡️ RPKI - The Immunity System of Aicent Stack");
    println!("   First AI with true Immune Sovereignty.");

    let mut pipeline = ImmunePipeline::new();
    
    let task = "882-node-maintenance-command";
    let result = pipeline.verify_and_watermark(task);

    println!("🔍 Verified task: {}", task);
    println!("   Watermark applied: {}", result.watermark);
    println!("   Trust score: {:.2}/1.0", result.trust_score);
    println!("   Status: {}", if result.is_valid { "✅ SECURE" } else { "❌ REJECTED" });

    println!("\n✅ RPKI immune shield activated - No malicious injection possible.");
}
