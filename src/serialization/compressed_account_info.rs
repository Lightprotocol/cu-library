use borsh::{BorshDeserialize, BorshSerialize};
use light_program_profiler::profile;
use light_zero_copy::{traits::ZeroCopyAt, ZeroCopy};
use pinocchio::program_error::ProgramError;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};
use wincode::containers::{self, Pod};
use wincode_derive::{SchemaRead, SchemaWrite};
use zerocopy::{Immutable, KnownLayout};

#[repr(C)]
#[derive(
    ZeroCopy,
    BorshDeserialize,
    BorshSerialize,
    borsh1::BorshDeserialize,
    borsh1::BorshSerialize,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    SchemaRead,
    SchemaWrite,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Default,
    Immutable,
    KnownLayout,
)]
#[archive(check_bytes)]
pub struct PackedMerkleContext {
    #[wincode(with = "Pod<_>")]
    pub merkle_tree_pubkey_index: u8,
    #[wincode(with = "Pod<_>")]
    pub nullifier_queue_pubkey_index: u8,
    #[wincode(with = "Pod<_>")]
    pub leaf_index: u32,
    #[wincode(with = "Pod<_>")]
    pub prove_by_index: bool,
}

#[derive(
    Debug,
    Default,
    PartialEq,
    Clone,
    BorshSerialize,
    BorshDeserialize,
    borsh1::BorshSerialize,
    borsh1::BorshDeserialize,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    SchemaRead,
    SchemaWrite,
    ZeroCopy,
)]
#[archive(check_bytes)]
#[repr(C)]
pub struct InAccountInfo {
    #[wincode(with = "Pod<_>")]
    pub discriminator: [u8; 8],
    /// Data hash
    #[wincode(with = "Pod<_>")]
    pub data_hash: [u8; 32],
    /// Merkle tree context.
    pub merkle_context: PackedMerkleContext,
    /// Root index.
    #[wincode(with = "Pod<_>")]
    pub root_index: u16,
    /// Lamports.
    #[wincode(with = "Pod<_>")]
    pub lamports: u64,
}

#[derive(
    Debug,
    Default,
    PartialEq,
    Clone,
    BorshSerialize,
    BorshDeserialize,
    borsh1::BorshSerialize,
    borsh1::BorshDeserialize,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    SchemaRead,
    SchemaWrite,
    ZeroCopy,
)]
#[archive(check_bytes)]
#[repr(C)]
pub struct OutAccountInfo {
    #[wincode(with = "Pod<_>")]
    pub discriminator: [u8; 8],
    /// Data hash
    #[wincode(with = "Pod<_>")]
    pub data_hash: [u8; 32],
    #[wincode(with = "Pod<_>")]
    pub output_merkle_tree_index: u8,
    /// Lamports.
    #[wincode(with = "Pod<_>")]
    pub lamports: u64,
    /// Account data.
    #[wincode(with = "containers::Vec<Pod<_>>")]
    pub data: Vec<u8>,
}

#[derive(
    Debug,
    PartialEq,
    Clone,
    Default,
    BorshSerialize,
    BorshDeserialize,
    borsh1::BorshSerialize,
    borsh1::BorshDeserialize,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    SchemaRead,
    SchemaWrite,
    ZeroCopy,
)]
#[archive(check_bytes)]
#[repr(C)]
pub struct CompressedAccountInfo {
    /// Address.
    pub address: Option<[u8; 32]>,
    /// Input account.
    pub input: Option<InAccountInfo>,
    /// Output account.
    pub output: Option<OutAccountInfo>,
}

fn create_test_data() -> CompressedAccountInfo {
    CompressedAccountInfo {
        address: Some([1u8; 32]),
        input: Some(InAccountInfo {
            discriminator: [1, 2, 3, 4, 5, 6, 7, 8],
            data_hash: [9u8; 32],
            merkle_context: PackedMerkleContext {
                merkle_tree_pubkey_index: 1,
                nullifier_queue_pubkey_index: 2,
                leaf_index: 12345,
                prove_by_index: true,
            },
            root_index: 42,
            lamports: 1_000_000_000,
        }),
        output: Some(OutAccountInfo {
            discriminator: [8, 7, 6, 5, 4, 3, 2, 1],
            data_hash: [10u8; 32],
            output_merkle_tree_index: 3,
            lamports: 2_000_000_000,
            data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        }),
    }
}

/// Helper function to create and serialize test data using Borsh
pub fn serialize_compressed_account_info() -> Vec<u8> {
    let test_data = create_test_data();
    borsh::to_vec(&test_data).unwrap()
}

/// Helper function to create and serialize test data using Wincode
pub fn serialize_compressed_account_info_wincode() -> Vec<u8> {
    let test_data = create_test_data();
    wincode::serialize(&test_data).unwrap()
}

/// Helper function to create and serialize test data using Bincode
pub fn serialize_compressed_account_info_bincode() -> Vec<u8> {
    let test_data = create_test_data();
    bincode::serialize(&test_data).unwrap()
}

/// Helper function to create and serialize test data using Borsh v1
pub fn serialize_compressed_account_info_borsh1() -> Vec<u8> {
    let test_data = create_test_data();
    borsh1::to_vec(&test_data).unwrap()
}

/// Helper function to create and serialize test data using Rkyv
pub fn serialize_compressed_account_info_rkyv() -> Vec<u8> {
    let test_data = create_test_data();
    rkyv::to_bytes::<_, 256>(&test_data).unwrap().to_vec()
}

#[profile]
pub fn borsh_deserialize(serialized_data: &[u8]) -> Result<CompressedAccountInfo, ProgramError> {
    CompressedAccountInfo::try_from_slice(serialized_data)
        .map_err(|_| ProgramError::InvalidAccountData)
}

#[profile]
pub fn zero_copy_deserialize<'a>(
    serialized_data: &'a [u8],
) -> Result<<CompressedAccountInfo as ZeroCopyAt<'a>>::ZeroCopyAt, ProgramError> {
    Ok(CompressedAccountInfo::zero_copy_at(serialized_data)
        .map_err(|e| ProgramError::from((u32::from(e)) as u64))?
        .0)
}

#[profile]
pub fn wincode_deserialize(serialized_data: &[u8]) -> Result<CompressedAccountInfo, ProgramError> {
    wincode::deserialize(serialized_data).map_err(|_| ProgramError::InvalidAccountData)
}

#[profile]
pub fn bincode_deserialize(serialized_data: &[u8]) -> Result<CompressedAccountInfo, ProgramError> {
    bincode::deserialize(serialized_data).map_err(|_| ProgramError::InvalidAccountData)
}

#[profile]
pub fn borsh1_deserialize(serialized_data: &[u8]) -> Result<CompressedAccountInfo, ProgramError> {
    borsh1::from_slice(serialized_data).map_err(|_| ProgramError::InvalidAccountData)
}

#[profile]
pub fn rkyv_zero_copy_deserialize(
    serialized_data: &[u8],
) -> Result<&rkyv::Archived<CompressedAccountInfo>, ProgramError> {
    rkyv::check_archived_root::<CompressedAccountInfo>(serialized_data)
        .map_err(|_| ProgramError::InvalidAccountData)
}
