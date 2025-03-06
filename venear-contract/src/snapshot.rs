use crate::*;

#[near]
impl Contract {
    /// View method. Returns the current snapshot of the Merkle tree and the global state.
    pub fn get_snapshot(&self) -> (MerkleTreeSnapshot, VGlobalState) {
        self.tree.get_snapshot().expect("Snapshot is not available")
    }

    /// View method. Returns the proof for the given account and the account value.
    pub fn get_proof(&self, account_id: AccountId) -> (MerkleProof, VAccount) {
        self.tree
            .get_proof(&account_id)
            .expect(format!("Account {} is not found", account_id).as_str())
    }
}
