use clap::Parser;
use serde_json::Value;
use sp1_sdk::ProverClient;
use std::error::Error;
use std::fs;
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;

/// Command line arguments for the Twine Solana consensus proof verifier
#[derive(Parser, Debug)]
#[clap(author, version, about = "Twine Solana Consensus Proof Verifier")]
struct Args {
    /// Path to the proof JSON file
    #[clap(short, long, default_value = "data/groth16_proof.json")]
    proof_path: String,

    /// Path to the verification key JSON file
    #[clap(short, long, default_value = "data/vkey.json")]
    vkey_path: String,
}

/// Extract a user-friendly error message from a panic payload
fn extract_error_message(panic_message: &str) -> String {
    if panic_message.contains("invalid point: subgroup check failed") {
        "The proof contains an invalid curve point (subgroup check failed). This usually means the proof is malformed or corrupted.".to_string()
    } else if panic_message.contains("failed to verify proof") {
        "The proof verification failed. The proof may be invalid or not match the verification key.".to_string()
    } else {
        let error_lines: Vec<&str> = panic_message.lines().collect();
        if !error_lines.is_empty() {
            format!("Verification error: {}", error_lines[0])
        } else {
            "Unknown verification error occurred".to_string()
        }
    }
}

/// Verify a Solana consensus proof using SP1
fn verify_proof(proof_path: &Path, vkey_path: &Path) -> Result<bool, Box<dyn Error>> {
    println!("Loading proof from: {}", proof_path.display());

    let proof_json = fs::read_to_string(proof_path)?;
    let proof: Value = serde_json::from_str(&proof_json)?;

    println!("Loading verification key from: {}", vkey_path.display());

    let vkey_json = fs::read_to_string(vkey_path)?;
    let vk: Value = serde_json::from_str(&vkey_json)?;

    let client = ProverClient::from_env()?; // Исправлено: теперь обрабатывается Result

    println!("Performing verification...");

    let verification_result = panic::catch_unwind(AssertUnwindSafe(|| client.verify(&proof, &vk)));

    match verification_result {
        Ok(result) => match result {
            Ok(_) => {
                println!("✅ VERIFICATION SUCCESSFUL: The Solana consensus proof is valid!");
                Ok(true)
            }
            Err(e) => {
                println!("❌ VERIFICATION FAILED: The Solana consensus proof is invalid.");
                println!("Error: {}", e);
                Ok(false)
            }
        },
        Err(panic_payload) => {
            let panic_message = panic_payload
                .downcast_ref::<String>()
                .map(|s| s.clone())
                .or_else(|| panic_payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "Unknown panic occurred during verification".to_string());

            let user_friendly_message = extract_error_message(&panic_message);

            println!("❌ VERIFICATION FAILED: The Solana consensus proof is invalid.");
            println!("Error: {}", user_friendly_message);

            println!("\nDetailed error information (for debugging):");
            println!("{}", panic_message);

            Ok(false)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Twine Solana Consensus Proof Verifier");
    println!("=====================================");

    let args = Args::parse();

    let proof_path = Path::new(&args.proof_path);
    let vkey_path = Path::new(&args.vkey_path);

    match verify_proof(proof_path, vkey_path) {
        Ok(true) => {
            println!("Verification completed successfully!");
            Ok(())
        }
        Ok(false) => {
            println!("Verification failed!");
            Err("Proof verification failed".into())
        }
        Err(e) => {
            println!("Error verifying proof: {}", e);
            Err(e)
        }
    }
}
