use std::fs;
use std::path::Path;

const FORBIDDEN_TERMS: &[&str] = &[
    "Premium",
    "Claim",
    "Policy",
    "Reserve",
    "Balance",
    "Loan",
    "Court",
    "Legal",
    "Duty",
    "Risk",
    "Score",
    "Fraud",
    "Capital",
    "Treasury",
    "Underwriting",
    "Actuarial",
    "Bank",
    "Insurance",
    "Regulator",
];

fn main() {
    println!("BIL ANALYZER REPORT\n");

    let core_dir = Path::new("crates/bil-core/src");
    
    let mut all_passed = true;

    // 1. Check no_std
    let lib_rs = fs::read_to_string(core_dir.join("lib.rs")).expect("Failed to read lib.rs");
    if lib_rs.contains("#![no_std]") {
        println!("[PASS] bil-core is no_std");
    } else {
        println!("[FAIL] bil-core is missing #![no_std]");
        all_passed = false;
    }

    // 1.5 Check forbid unsafe
    if lib_rs.contains("#![forbid(unsafe_code)]") {
        println!("[PASS] bil-core forbids unsafe");
    } else {
        println!("[FAIL] bil-core is missing #![forbid(unsafe_code)]");
        all_passed = false;
    }

    // 2. Check forbidden terms
    let mut found_forbidden = false;
    for entry in fs::read_dir(core_dir).expect("Failed to read core dir") {
        let entry = entry.expect("Failed to read entry");
        if entry.path().extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(entry.path()).expect("Failed to read file");
            for term in FORBIDDEN_TERMS {
                if content.contains(term) {
                    println!("[FAIL] bil-core contains forbidden semantic identifier: {}", term);
                    found_forbidden = true;
                    all_passed = false;
                }
            }
        }
    }
    if !found_forbidden {
        println!("[PASS] bil-core contains no forbidden semantic identifiers");
    }

    // 3. Check dependencies
    let cargo_toml = fs::read_to_string("crates/bil-core/Cargo.toml").expect("Failed to read Cargo.toml");
    if cargo_toml.contains("bank") || cargo_toml.contains("insurance") || cargo_toml.contains("legal") {
        println!("[FAIL] bil-core imports domain crates");
        all_passed = false;
    } else {
        println!("[PASS] bil-core imports no domain crates");
    }

    // 4. Check examples depend on bil-core
    let mut examples_depend = true;
    for example in &["bank-projection", "insurance-projection", "legal-projection", "observation-algebra-demo"] {
        let example_toml = fs::read_to_string(format!("examples/{}/Cargo.toml", example)).expect("Failed to read example Cargo.toml");
        if !example_toml.contains("bil-core") {
            println!("[FAIL] example {} does not depend on bil-core", example);
            examples_depend = false;
            all_passed = false;
        }
    }
    if examples_depend {
        println!("[PASS] examples depend on bil-core");
    }

    // 5. Check tests exist
    let core_lib = fs::read_to_string("crates/bil-core/src/lib.rs").expect("Failed to read core lib");
    if core_lib.contains("#[test]") && core_lib.contains("draft_to_anchored_lifecycle_works") {
        println!("[PASS] receipt lifecycle tests present");
    } else {
        println!("[FAIL] receipt lifecycle tests missing");
        all_passed = false;
    }

    let codec_lib = fs::read_to_string("crates/bil-codec/src/lib.rs").expect("Failed to read codec lib");
    let crypto_lib = fs::read_to_string("crates/bil-crypto/src/lib.rs").expect("Failed to read crypto lib");
    if codec_lib.contains("#[test]") && crypto_lib.contains("#[test]") {
        println!("[PASS] codec/hash tests present");
    } else {
        println!("[FAIL] codec/hash tests missing");
        all_passed = false;
    }

    if Path::new("examples/observation-algebra-demo/src/main.rs").exists() {
        println!("[PASS] projection demo present");
    } else {
        println!("[FAIL] projection demo missing");
        all_passed = false;
    }

    if all_passed {
        println!("\nRESULT: HOLONOMY PRESERVED");
        std::process::exit(0);
    } else {
        println!("\nRESULT: HOLONOMY FAILED");
        std::process::exit(1);
    }
}
