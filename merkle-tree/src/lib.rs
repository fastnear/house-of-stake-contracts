use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::Base58CryptoHash;
use near_sdk::store::LookupMap;
use near_sdk::{borsh, BlockHeight, CryptoHash, IntoStorageKey};
use near_sdk::{near, AccountId, BorshStorageKey};

#[derive(BorshStorageKey)]
#[near(serializers=[borsh])]
enum MerkleStorageKeys {
    Hashes,
    Data,
    Accounts,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
#[near(serializers=[borsh])]
pub struct HeightAndIndex {
    pub height: u8,
    pub index: u32,
}

/// Persistent Merkle Tree.
///
/// It stores intermediate hashes and leaves in the persistent storage.
/// We assume height zero is where the leaves are stored. The height increases as we go up the tree.
/// When we add a new leaf, we add it to the end of the leaves array and update the tree.
/// When the number of leaves grows to 2^height, we add a new level to the tree.
/// We also store hashes of the leaves in the tree to make it easier to verify the proofs.
/// To hash the leaf, we serialize it using Borsh and then hash the serialized bytes using sha256.
/// The empty leaf hash is by all zeros hash: `[0u8; 32]`.
/// Note, that Value `T` has to contain the `account_id` in order to be able to verify the proof.
/// The `global` field is used to store the global state of the tree. E.g. total sum of balances.
/// When we save the previous snapshot, we also save the global state at that time.
#[near(serializers=[borsh])]
pub struct MerkleTree<V, G>
where
    V: BorshSerialize + BorshDeserialize + Clone,
    G: BorshSerialize + BorshDeserialize + Clone,
{
    pub(crate) root: CryptoHash,
    pub(crate) length: u32,
    pub(crate) hashes: LookupMap<HeightAndIndex, CryptoHash>,
    pub(crate) data: LookupMap<u32, V>,
    pub(crate) accounts: LookupMap<AccountId, u32>,
    /// The global state of the tree. E.g. total sum of balances.
    pub(crate) global_state: G,
    pub(crate) previous_snapshot: Option<(MerkleTreeSnapshot, G)>,
    pub(crate) last_block_height: BlockHeight,
}

impl<V, G> MerkleTree<V, G>
where
    V: BorshSerialize + BorshDeserialize + Clone,
    G: BorshSerialize + BorshDeserialize + Clone,
{
    pub fn new<S>(storage_key_prefix: S, global_state: G) -> Self
    where
        S: IntoStorageKey,
    {
        let prefix = storage_key_prefix.into_storage_key();

        Self {
            root: CryptoHash::default(),
            length: 0,
            hashes: LookupMap::new(
                [
                    &prefix[..],
                    &MerkleStorageKeys::Hashes.into_storage_key()[..],
                ]
                .concat(),
            ),
            data: LookupMap::new(
                [&prefix[..], &MerkleStorageKeys::Data.into_storage_key()[..]].concat(),
            ),
            accounts: LookupMap::new(
                [
                    &prefix[..],
                    &MerkleStorageKeys::Accounts.into_storage_key()[..],
                ]
                .concat(),
            ),
            global_state,
            previous_snapshot: None,
            last_block_height: near_sdk::env::block_height(),
        }
    }

    /// An internal method to potentially update previous snapshot before we do any changes to the
    /// merkle tree data.
    fn internal_pre_update(&mut self) {
        let block_height = near_sdk::env::block_height();
        if self.last_block_height != block_height {
            self.previous_snapshot = Some((
                MerkleTreeSnapshot {
                    root: self.root.into(),
                    length: self.length,
                    block_height: self.last_block_height,
                },
                self.global_state.clone(),
            ));
            self.last_block_height = block_height;
        }
    }

    fn internal_hash_value(&self, index: u32) -> CryptoHash {
        let data = self
            .data
            .get(&index)
            .map(|data| borsh::to_vec(data).expect("Failed to serialize data"));
        data.map(|data| near_sdk::env::sha256(&data).try_into().unwrap())
            .unwrap_or(CryptoHash::default())
    }

    fn tree_height(&self) -> u8 {
        if self.length == 0 {
            0
        } else {
            self.length.ilog2() as u8 + 1
        }
    }

    fn internal_get_hash(&self, height: u8, index: u32) -> CryptoHash {
        self.hashes
            .get(&HeightAndIndex { height, index })
            .cloned()
            .unwrap_or_else(|| CryptoHash::default())
    }

    fn internal_set_hash(&mut self, height: u8, index: u32, hash: CryptoHash) {
        self.hashes.insert(HeightAndIndex { height, index }, hash);
    }

    fn internal_post_update(&mut self, index: u32) {
        let hash = self.internal_hash_value(index);
        self.internal_set_hash(0, index, hash);
        for height in 1..self.tree_height() {
            let height_index = index >> height;
            let left_hash = self.internal_get_hash(height - 1, height_index << 1);
            let right_hash = self.internal_get_hash(height - 1, (height_index << 1) + 1);
            let concat = [&left_hash[..], &right_hash[..]].concat();
            let hash = near_sdk::env::sha256(&concat).try_into().unwrap();
            self.internal_set_hash(height, height_index, hash);
        }
        self.root = self.internal_get_hash(self.tree_height() - 1, 0);
    }

    /// Returns the previous snapshot if it exists.
    pub fn get_snapshot(&self) -> Option<(MerkleTreeSnapshot, G)> {
        let block_height = near_sdk::env::block_height();
        if self.last_block_height != block_height {
            Some((
                MerkleTreeSnapshot {
                    root: self.root.into(),
                    length: self.length,
                    block_height: self.last_block_height,
                },
                self.global_state.clone(),
            ))
        } else {
            self.previous_snapshot.clone()
        }
    }

    pub fn get_global_state(&self) -> &G {
        &self.global_state
    }

    pub fn set_global_state(&mut self, global_state: G) {
        self.internal_pre_update();
        self.global_state = global_state;
    }

    /// Returns the value for the given account_id.
    pub fn get(&self, account_id: &AccountId) -> Option<&V> {
        self.accounts
            .get(account_id)
            .and_then(|index| self.data.get(&index))
    }

    /// Sets the value for the given account_id and returns the old value if it existed.
    pub fn set(&mut self, account_id: AccountId, new_value: V) -> Option<V> {
        self.internal_pre_update();
        let index = self.accounts.get(&account_id).cloned().unwrap_or_else(|| {
            let index = self.length;
            self.length += 1;
            self.accounts.insert(account_id.clone(), index);
            index
        });
        let old_value = self.data.insert(index, new_value);
        self.internal_post_update(index);
        old_value
    }

    pub fn get_proof(&self, account_id: &AccountId) -> Option<(MerkleProof, V)> {
        let &index = self.accounts.get(account_id)?;
        let mut path = vec![];
        for height in 0..self.tree_height() - 1 {
            let height_index = index >> height;
            let sibling_index = height_index ^ 1;
            let sibling_hash = self.internal_get_hash(height, sibling_index);
            path.push(sibling_hash.into());
        }
        Some((
            MerkleProof { index, path },
            self.data.get(&index).cloned().unwrap(),
        ))
    }

    pub fn len(&self) -> u32 {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn get_by_index(&self, index: u32) -> Option<&V> {
        self.data.get(&index)
    }
}

#[derive(Clone)]
#[near(serializers=[borsh, json])]
pub struct MerkleProof {
    pub index: u32,
    pub path: Vec<Base58CryptoHash>,
}

#[derive(Clone)]
#[near(serializers=[borsh, json])]
pub struct MerkleTreeSnapshot {
    pub root: Base58CryptoHash,
    pub length: u32,
    pub block_height: BlockHeight,
}

impl MerkleProof {
    pub fn verify<T>(&self, root: CryptoHash, length: u32, value: &T) -> bool
    where
        T: BorshSerialize,
    {
        if self.index >= length {
            return false;
        }
        // The length is greater than 0
        let tree_height = length.ilog2() + 1;
        if self.path.len() + 1 != tree_height as usize {
            return false;
        }

        let data = borsh::to_vec(value).expect("Failed to serialize");
        let mut hash: CryptoHash = near_sdk::env::sha256(&data).try_into().unwrap();

        for (height, sibling_hash) in self.path.iter().enumerate() {
            let sibling_hash: CryptoHash = sibling_hash.clone().into();
            let height_index = self.index >> height;
            let concat = if height_index & 1 == 0 {
                [&hash[..], &sibling_hash[..]].concat()
            } else {
                [&sibling_hash[..], &hash[..]].concat()
            };
            hash = near_sdk::env::sha256(&concat).try_into().unwrap();
        }
        hash == root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    #[derive(BorshStorageKey)]
    #[near]
    enum StorageKeys {
        Tree,
    }

    #[test]
    fn test_merkle_tree() {
        let mut context = VMContextBuilder::new().build();
        testing_env!(context.clone());

        let global_state = "global_state".to_string();
        let mut tree = MerkleTree::new(StorageKeys::Tree, global_state.clone());
        assert_eq!(tree.tree_height(), 0);
        assert_eq!(tree.root, CryptoHash::default());
        assert_eq!(tree.length, 0);

        context.block_index += 1;
        testing_env!(context.clone());

        let value = 42u32;
        let account_id: AccountId = "alice.near".parse().unwrap();
        let old_value = tree.set(account_id.clone(), value);
        assert_eq!(old_value, None);
        assert_eq!(tree.tree_height(), 1);
        assert_ne!(tree.root, CryptoHash::default());
        assert_eq!(tree.length, 1);

        context.block_index += 1;
        testing_env!(context.clone());

        let old_value = tree.set(account_id.clone(), value + 1);
        assert_eq!(old_value, Some(value));
        assert_eq!(tree.tree_height(), 1);
        assert_ne!(tree.root, CryptoHash::default());
        assert_eq!(tree.length, 1);

        context.block_index += 1;
        testing_env!(context.clone());

        let (snapshot, gs) = tree.get_snapshot().unwrap();
        assert_eq!(global_state, gs);
        let (proof, account) = tree.get_proof(&account_id).unwrap();
        assert_eq!(account, value + 1);
        assert!(proof.verify(snapshot.root.into(), snapshot.length, &account));
    }
}
