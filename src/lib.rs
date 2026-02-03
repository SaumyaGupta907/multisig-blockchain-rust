use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

// Transaction types - showcasing different blockchain functionalities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Standard {
        from: String,
        to: String,
        amount: f64,
    },
    MultiSig {
        from: String,
        to: String,
        amount: f64,
        required_signatures: usize,
        signatures: Vec<String>, // In real blockchain, these would be cryptographic signatures
    },
    TimeLocked {
        from: String,
        to: String,
        amount: f64,
        unlock_time: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub tx_type: TransactionType,
    pub timestamp: DateTime<Utc>,
    pub nonce: u64,
}

impl Transaction {
    pub fn new(tx_type: TransactionType, nonce: u64) -> Self {
        let timestamp = Utc::now();
        let id = Self::calculate_hash(&tx_type, timestamp, nonce);
        
        Transaction {
            id,
            tx_type,
            timestamp,
            nonce,
        }
    }

    fn calculate_hash(tx_type: &TransactionType, timestamp: DateTime<Utc>, nonce: u64) -> String {
        let data = format!("{:?}{}{}", tx_type, timestamp, nonce);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn is_valid(&self, current_time: DateTime<Utc>) -> Result<(), String> {
        match &self.tx_type {
            TransactionType::MultiSig { required_signatures, signatures, .. } => {
                if signatures.len() < *required_signatures {
                    return Err(format!(
                        "Insufficient signatures: {} required, {} provided",
                        required_signatures,
                        signatures.len()
                    ));
                }
            }
            TransactionType::TimeLocked { unlock_time, .. } => {
                if current_time < *unlock_time {
                    return Err(format!(
                        "Transaction locked until {}",
                        unlock_time
                    ));
                }
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now();
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            serde_json::to_string(&self.transactions).unwrap(),
            self.previous_hash,
            self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }

    // Proof of Work - mining with difficulty
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        
        println!("Block mined: {} (nonce: {})", self.hash, self.nonce);
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub balances: HashMap<String, f64>,
    pub multisig_wallets: HashMap<String, Vec<String>>, // wallet_id -> authorized signers
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty,
            pending_transactions: Vec::new(),
            balances: HashMap::new(),
            multisig_wallets: HashMap::new(),
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_transaction = Transaction::new(
            TransactionType::Standard {
                from: "genesis".to_string(),
                to: "genesis".to_string(),
                amount: 0.0,
            },
            0,
        );
        
        let mut genesis_block = Block::new(0, vec![genesis_transaction], "0".to_string());
        genesis_block.mine_block(self.difficulty);
        self.chain.push(genesis_block);
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        // Validate transaction before adding
        transaction.is_valid(Utc::now())?;
        
        // Check balance for non-genesis transactions
        match &transaction.tx_type {
            TransactionType::Standard { from, amount, .. }
            | TransactionType::MultiSig { from, amount, .. }
            | TransactionType::TimeLocked { from, amount, .. } => {
                if from != "genesis" {
                    let balance = self.balances.get(from).unwrap_or(&0.0);
                    if balance < amount {
                        return Err(format!("Insufficient balance: {} has {}", from, balance));
                    }
                }
            }
        }
        
        self.pending_transactions.push(transaction);
        Ok(())
    }

    pub fn mine_pending_transactions(&mut self) {
        if self.pending_transactions.is_empty() {
            println!("No transactions to mine");
            return;
        }

        let previous_hash = self.chain.last().unwrap().hash.clone();
        let index = self.chain.len() as u64;
        
        let mut block = Block::new(index, self.pending_transactions.clone(), previous_hash);
        block.mine_block(self.difficulty);
        
        // Update balances
        for tx in &block.transactions {
            match &tx.tx_type {
                TransactionType::Standard { from, to, amount }
                | TransactionType::MultiSig { from, to, amount, .. }
                | TransactionType::TimeLocked { from, to, amount, .. } => {
                    if from != "genesis" {
                        *self.balances.entry(from.clone()).or_insert(0.0) -= amount;
                    }
                    *self.balances.entry(to.clone()).or_insert(0.0) += amount;
                }
            }
        }
        
        self.chain.push(block);
        self.pending_transactions.clear();
    }

    pub fn create_multisig_wallet(&mut self, wallet_id: String, signers: Vec<String>) {
        self.multisig_wallets.insert(wallet_id, signers);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // Verify hash integrity
            if current_block.hash != current_block.calculate_hash() {
                println!("Block {} hash is invalid", i);
                return false;
            }

            // Verify chain linkage
            if current_block.previous_hash != previous_block.hash {
                println!("Block {} is not properly linked", i);
                return false;
            }

            // Verify proof of work
            if !current_block.hash.starts_with(&"0".repeat(self.difficulty)) {
                println!("Block {} doesn't meet difficulty requirement", i);
                return false;
            }
        }
        true
    }

    pub fn get_balance(&self, address: &str) -> f64 {
        *self.balances.get(address).unwrap_or(&0.0)
    }

    pub fn print_chain(&self) {
        println!("\n========== BLOCKCHAIN STATE ==========");
        for block in &self.chain {
            println!("\nBlock #{}", block.index);
            println!("Timestamp: {}", block.timestamp);
            println!("Hash: {}", block.hash);
            println!("Previous Hash: {}", block.previous_hash);
            println!("Nonce: {}", block.nonce);
            println!("Transactions: {}", block.transactions.len());
            for tx in &block.transactions {
                println!("  - {:?}", tx.tx_type);
            }
        }
        println!("\n========== BALANCES ==========");
        for (address, balance) in &self.balances {
            println!("{}: {}", address, balance);
        }
        println!("=====================================\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_creation() {
        let blockchain = Blockchain::new(2);
        assert_eq!(blockchain.chain.len(), 1);
        assert!(blockchain.is_chain_valid());
    }

    #[test]
    fn test_standard_transaction() {
        let mut blockchain = Blockchain::new(2);
        
        // Add initial funds
        let tx1 = Transaction::new(
            TransactionType::Standard {
                from: "genesis".to_string(),
                to: "Alice".to_string(),
                amount: 100.0,
            },
            1,
        );
        blockchain.add_transaction(tx1).unwrap();
        blockchain.mine_pending_transactions();
        
        assert_eq!(blockchain.get_balance("Alice"), 100.0);
    }

    #[test]
    fn test_multisig_transaction() {
        let mut blockchain = Blockchain::new(2);
        
        // Create multisig wallet
        blockchain.create_multisig_wallet(
            "vault".to_string(),
            vec!["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()],
        );
        
        // Fund the vault
        let tx1 = Transaction::new(
            TransactionType::Standard {
                from: "genesis".to_string(),
                to: "vault".to_string(),
                amount: 1000.0,
            },
            1,
        );
        blockchain.add_transaction(tx1).unwrap();
        blockchain.mine_pending_transactions();
        
        // Create multisig transaction with 2 of 3 signatures
        let tx2 = Transaction::new(
            TransactionType::MultiSig {
                from: "vault".to_string(),
                to: "Dave".to_string(),
                amount: 500.0,
                required_signatures: 2,
                signatures: vec!["Alice".to_string(), "Bob".to_string()],
            },
            2,
        );
        blockchain.add_transaction(tx2).unwrap();
        blockchain.mine_pending_transactions();
        
        assert_eq!(blockchain.get_balance("Dave"), 500.0);
        assert_eq!(blockchain.get_balance("vault"), 500.0);
    }

    #[test]
    fn test_time_locked_transaction() {
        let mut blockchain = Blockchain::new(2);
        
        // Create time-locked transaction (unlocks in the past for testing)
        let unlock_time = Utc::now() - chrono::Duration::hours(1);
        let tx = Transaction::new(
            TransactionType::TimeLocked {
                from: "genesis".to_string(),
                to: "Alice".to_string(),
                amount: 100.0,
                unlock_time,
            },
            1,
        );
        
        assert!(blockchain.add_transaction(tx).is_ok());
    }
}
