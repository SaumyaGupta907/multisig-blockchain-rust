use multisig_blockchain::*;
use chrono::{Duration, Utc};

fn main() {
    println!("🔗 Multi-Signature Blockchain with Time-Locked Transactions\n");
    
    // Create blockchain with difficulty 3 (3 leading zeros in hash)
    let mut blockchain = Blockchain::new(3);
    
    println!("✓ Genesis block created\n");
    
    // Demo 1: Standard Transactions
    println!("=== DEMO 1: Standard Transactions ===");
    let tx1 = Transaction::new(
        TransactionType::Standard {
            from: "genesis".to_string(),
            to: "Alice".to_string(),
            amount: 1000.0,
        },
        1,
    );
    blockchain.add_transaction(tx1).unwrap();
    
    let tx2 = Transaction::new(
        TransactionType::Standard {
            from: "genesis".to_string(),
            to: "Bob".to_string(),
            amount: 500.0,
        },
        2,
    );
    blockchain.add_transaction(tx2).unwrap();
    
    println!("Mining block with 2 transactions...");
    blockchain.mine_pending_transactions();
    println!("✓ Block mined successfully\n");
    
    // Demo 2: Multi-Signature Wallet
    println!("=== DEMO 2: Multi-Signature Wallet ===");
    println!("Creating a 2-of-3 multisig wallet for company treasury...");
    
    blockchain.create_multisig_wallet(
        "company_treasury".to_string(),
        vec![
            "CEO".to_string(),
            "CFO".to_string(),
            "CTO".to_string(),
        ],
    );
    
    // Fund the treasury
    let tx3 = Transaction::new(
        TransactionType::Standard {
            from: "genesis".to_string(),
            to: "company_treasury".to_string(),
            amount: 10000.0,
        },
        3,
    );
    blockchain.add_transaction(tx3).unwrap();
    blockchain.mine_pending_transactions();
    
    println!("✓ Treasury funded with 10,000 units\n");
    
    // Attempt multisig withdrawal
    println!("Attempting withdrawal requiring 2 signatures (CEO + CFO)...");
    let tx4 = Transaction::new(
        TransactionType::MultiSig {
            from: "company_treasury".to_string(),
            to: "Vendor".to_string(),
            amount: 3000.0,
            required_signatures: 2,
            signatures: vec!["CEO".to_string(), "CFO".to_string()],
        },
        4,
    );
    blockchain.add_transaction(tx4).unwrap();
    blockchain.mine_pending_transactions();
    println!("✓ Multisig transaction approved and executed\n");
    
    // Demo 3: Time-Locked Transaction
    println!("=== DEMO 3: Time-Locked Transactions ===");
    println!("Creating vesting schedule for employee stock options...");
    
    // Transaction that unlocks in 1 year (simulated as past for demo)
    let unlock_time = Utc::now() - Duration::days(1); // Past date for demo
    let tx5 = Transaction::new(
        TransactionType::TimeLocked {
            from: "company_treasury".to_string(),
            to: "Employee_John".to_string(),
            amount: 1000.0,
            unlock_time,
        },
        5,
    );
    
    println!("Transaction will unlock at: {}", unlock_time);
    blockchain.add_transaction(tx5).unwrap();
    blockchain.mine_pending_transactions();
    println!("✓ Time-locked transaction executed (vesting period passed)\n");
    
    // Demo 4: Transaction Validation
    println!("=== DEMO 4: Transaction Validation ===");
    
    // Try insufficient signatures
    println!("Attempting multisig with only 1 signature (should fail)...");
    let tx6 = Transaction::new(
        TransactionType::MultiSig {
            from: "company_treasury".to_string(),
            to: "Hacker".to_string(),
            amount: 5000.0,
            required_signatures: 2,
            signatures: vec!["CEO".to_string()], // Only 1 signature
        },
        6,
    );
    
    match blockchain.add_transaction(tx6) {
        Ok(_) => println!("Transaction added"),
        Err(e) => println!("✓ Transaction rejected: {}", e),
    }
    
    // Try future-locked transaction
    println!("\nAttempting time-locked transaction (future unlock)...");
    let future_unlock = Utc::now() + Duration::days(365);
    let tx7 = Transaction::new(
        TransactionType::TimeLocked {
            from: "Alice".to_string(),
            to: "Bob".to_string(),
            amount: 100.0,
            unlock_time: future_unlock,
        },
        7,
    );
    
    match blockchain.add_transaction(tx7) {
        Ok(_) => println!("Transaction added"),
        Err(e) => println!("✓ Transaction rejected: {}", e),
    }
    println!();
    
    // Demo 5: Chain Validation
    println!("=== DEMO 5: Blockchain Validation ===");
    println!("Validating entire blockchain integrity...");
    
    if blockchain.is_chain_valid() {
        println!("✓ Blockchain is valid!");
        println!("  - All hashes verified");
        println!("  - Chain linkage intact");
        println!("  - Proof of work satisfied");
    } else {
        println!("✗ Blockchain validation failed!");
    }
    
    // Print final state
    blockchain.print_chain();
    
    // Summary
    println!("\n=== BLOCKCHAIN SUMMARY ===");
    println!("Total Blocks: {}", blockchain.chain.len());
    println!("Mining Difficulty: {} leading zeros", blockchain.difficulty);
    println!("Total Accounts: {}", blockchain.balances.len());
    println!("\nKey Features Demonstrated:");
    println!("  ✓ Proof of Work mining");
    println!("  ✓ Multi-signature wallets (2-of-3)");
    println!("  ✓ Time-locked transactions (vesting)");
    println!("  ✓ Transaction validation");
    println!("  ✓ Chain integrity verification");
    println!("  ✓ Balance tracking");
    
    println!("\n🎉 Blockchain demonstration complete!");
}
