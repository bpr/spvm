#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::collections::HashMap;

/* Future work
#[derive(Error, Debug)]
enum SPVMError {
    #[error("token already initialized")]
    TokenAlreadyInitialized,
    #[error("token not initialized")]
    TokenNotInitialized,
    #[error("insufficient balance")]
    InsufficientBalance,
    #[error("invalid transaction type")]
    InvalidTransactionType,
    #[error("mismatched noncess")]
    NonceMismatch,
    #[error("unknown SPVM error")]
    Unknown,
}
*/
pub struct SignatureChecker {}
impl SignatureChecker {
    /**
     * @dev Checks if a signature is valid for a given signer and data hash. If the signer is a smart contract, the
     * signature is validated against that smart contract using ERC-1271, otherwise it's validated using `ECDSA.recover`.
     *
     * NOTE: Unlike ECDSA signatures, contract signatures are revocable, and the outcome of this function can thus
     * change through time. It could return true at block N and false at block N+1 (or the opposite).
     */
    /*
    function isValidSignatureNow(address signer, bytes32 hash, bytes memory signature) internal view returns (bool) {
        if (signer.code.length == 0) {
            (address recovered, ECDSA.RecoverError err, ) = ECDSA.tryRecover(hash, signature);
            return err == ECDSA.RecoverError.NoError && recovered == signer;
        } else {
            return isValidERC1271SignatureNow(signer, hash, signature);
        }
    }
    */
    #[allow(unused_variables)]
    pub fn is_valid_signature_now(
        expected_signer: Address,
        message_hash: Bytes32,
        signature: Vec<u8>,
    ) -> bool {
        if is_smart_contract(&expected_signer) {
            false // stub
        } else {
            // let recovered = ECDSA::recover(message_hash, signature);
            // return recovered == expected_signer;
            true
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Address(pub [u8; 20]);

#[allow(unused_variables)]
fn is_smart_contract(signer: &Address) -> bool {
    false // stub
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Bytes32(pub [u8; 32]);

#[derive(Serialize, Deserialize, Clone, Debug)]
struct TransactionContent {
    from: Address,
    tx_type: u8,       // only first 2 bits used
    tx_param: Vec<u8>, // abi encoded parameters
    nonce: u32,
}

#[derive(Serialize, Deserialize, Clone,  PartialEq, Eq, Debug)]
struct MintTransactionParams {
    token_ticker: String,
    owner: Address,
    supply: u16,
}

#[derive(Serialize, Deserialize, Clone,  PartialEq, Eq, Debug)]
struct TransferTransactionParams {
    token_ticker: String,
    to: Address,
    amount: u16,
}

///  Block is a stub for now
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Block {
    transactions: Vec<SPVMTransaction>,
    block_hash: Bytes32,
    parent_hash: Bytes32,
    block_number: u32,
    proposer: Address,
    proposer_signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SPVMTransaction {
    tx_content: TransactionContent,
    transaction_hash: Bytes32,
    signature: Vec<u8>,
}

///  ElectionInterface is a stub for now
pub struct ElectionInterface {}

///  Slashing is a stub for now
pub struct Slashing {}

struct SPVM {
    initialized_tickers: HashMap<String, bool>,
    state: HashMap<String, HashMap<Address, u16>>,
    nonces: HashMap<Address, u32>,
    blocks: HashMap<u32, Block>,
    block_number: u32,
}

impl SPVM {
    pub fn new() -> Self {
        SPVM {
            initialized_tickers: HashMap::new(),
            state: HashMap::new(),
            nonces: HashMap::new(),
            blocks: HashMap::new(),
            block_number: 0,
        }
    }

    pub fn set_balance(&mut self, token_ticker: String, holder_address: Address, balance: u16) {
        self.initialized_tickers.insert(token_ticker.clone(), true);
        let nested_map = self.state.entry(token_ticker.clone()).or_default();
        nested_map.insert(holder_address, balance);
    }

    pub fn get_balance(&self, token_ticker: String, holder_address: Address) -> u16 {
        *self
            .state
            .get(&token_ticker)
            .unwrap()
            .get(&holder_address)
            .unwrap()
    }

    pub fn execute_raw_transaction(
        &mut self,
        raw_tx: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_tx_str = String::from_utf8(raw_tx).expect("param should be valid utf8");
        let tx_content: TransactionContent = serde_json::from_str(&raw_tx_str)?;
        if let Err(_) = self.check_validity(&tx_content) {
            return Err("Invalid transaction".into());
        }

        let tx_content_ref = &tx_content;
        if tx_content_ref.tx_type == 0 {
            let tx_param_str = String::from_utf8(tx_content_ref.tx_param.clone())
                .expect("param should be valid utf8");
            let mint_params: MintTransactionParams = serde_json::from_str(&tx_param_str).unwrap();
            self.set_balance(
                mint_params.token_ticker,
                mint_params.owner,
                mint_params.supply,
            );
        } else if tx_content_ref.tx_type == 1 {
            let tx_param_str = String::from_utf8(tx_content_ref.tx_param.clone())
                .expect("param should be valid utf8");
            let transfer_params: TransferTransactionParams =
                serde_json::from_str(&tx_param_str).unwrap();
            let from_balance = self.get_balance(
                transfer_params.token_ticker.clone(),
                tx_content.from.clone(),
            );
            self.set_balance(
                transfer_params.token_ticker.clone(),
                tx_content_ref.from.clone(),
                from_balance - transfer_params.amount,
            );
            let to_balance = self.get_balance(
                transfer_params.token_ticker.clone(),
                transfer_params.to.clone(),
            );
            self.set_balance(
                transfer_params.token_ticker.clone(),
                transfer_params.to.clone(),
                to_balance + transfer_params.amount,
            );
        }

        let from_nonce = self.nonces.get_mut(&tx_content_ref.from).unwrap();
        *from_nonce += 1;

        Ok(())
    }

    // There's no point in returning a bool here, since Result is already an enum with Ok and Err variants
    fn check_validity(
        &self,
        tx_content: &TransactionContent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if tx_content.tx_type == 0 {
            let tx_param_str =
                String::from_utf8(tx_content.tx_param.clone()).expect("param should be valid utf8");
            let mint_params: MintTransactionParams = serde_json::from_str(&tx_param_str).unwrap();
            let is_initialized_ticker = self
                .initialized_tickers
                .get(&mint_params.token_ticker)
                .unwrap();
            if *is_initialized_ticker {
                // WTF?
                return Err("TokenAlreadyInitialized".into());
            }
        } else if tx_content.tx_type == 1 {
            let tx_param_str =
                String::from_utf8(tx_content.tx_param.clone()).expect("param should be valid utf8");
            let transfer_params: TransferTransactionParams =
                serde_json::from_str(&tx_param_str).unwrap();
            let is_initialized_ticker = self
                .initialized_tickers
                .get(&transfer_params.token_ticker)
                .unwrap();
            if !*is_initialized_ticker {
                return Err("Token not initialized".into());
            };
            let nested_map = self.state.get(&transfer_params.token_ticker).unwrap();
            let balance = nested_map.get(&tx_content.from).unwrap();
            if *balance < transfer_params.amount {
                return Err("Insufficient balance".into());
            }
        } else {
            return Err("InvalidTransactionType".into());
        }

        let from_nonce = self.nonces.get(&tx_content.from).unwrap();
        if *from_nonce != tx_content.nonce {
            return Err("NonceMismatch".into());
        }

        Ok(())
    }

    pub fn validate_signature(
        &self,
        message_hash: Bytes32,
        signature: Vec<u8>,
        expected_signer: Address,
    ) -> bool {
        SignatureChecker::is_valid_signature_now(expected_signer, message_hash, signature)
    }

    pub fn execute_transaction(
        &mut self,
        txn: SPVMTransaction,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tx_hash = Bytes32(
            Keccak256::digest(&serde_json::to_string(&txn.tx_content).unwrap().as_bytes()).into(),
        );

        if tx_hash != txn.transaction_hash {
            return Err("Invalid transaction hash".into());
        }

        if !self.validate_signature(tx_hash, txn.signature.clone(), txn.tx_content.from.clone()) {
            return Err("Invalid signature".into());
        }

        self.execute_raw_transaction(
            serde_json::to_string(&txn.tx_content)
                .unwrap()
                .as_bytes()
                .to_vec(),
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_balance() {
        let mut spvm = SPVM::new();
        let token_ticker = "ABC".to_string();
        let holder_address = Address([1; 20]);
        let balance = 100;
        spvm.set_balance(token_ticker.clone(), holder_address.clone(), balance);
        assert_eq!(spvm.get_balance(token_ticker.clone(), holder_address.clone()), balance);
    }

    #[test]
    fn validate_signature() {
        // Rather useless now, always returns true
        let spvm = SPVM::new();
        let message_hash = Bytes32([1; 32]);
        let signature = vec![1, 2, 3];
        let expected_signer = Address([1; 20]);
        assert!(spvm.validate_signature(message_hash, signature, expected_signer));
    }

    #[test]
    fn execute_transaction() {
        let mut _spvm = SPVM::new();
        assert!(true);
    }
}
