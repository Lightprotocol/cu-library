use light_program_profiler::profile;
use pinocchio::program_error::ProgramError;

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, Debug)]
pub struct CompressedAccountPod {
    pub address: [u8; 32],
    pub discriminator: [u8; 8],
    pub data_hash: [u8; 32],
    pub leaf_index: u32,
    pub root_index: u16,
    pub _padding: [u8; 2],
    pub lamports: u64,
}

pub fn serialize_pod() -> Vec<u8> {
    let v = CompressedAccountPod {
        address: [1u8; 32],
        discriminator: [2u8; 8],
        data_hash: [3u8; 32],
        leaf_index: 12345,
        root_index: 42,
        _padding: [0u8; 2],
        lamports: 1_000_000_000,
    };
    bytemuck::bytes_of(&v).to_vec()
}

#[profile]
pub fn try_pod_read_unaligned(data: &[u8]) -> Result<CompressedAccountPod, ProgramError> {
    bytemuck::try_pod_read_unaligned::<CompressedAccountPod>(data)
        .map_err(|_| ProgramError::InvalidAccountData)
}

#[profile]
pub fn pod_read_unaligned(data: &[u8]) -> CompressedAccountPod {
    bytemuck::pod_read_unaligned::<CompressedAccountPod>(data)
}

#[profile]
pub fn try_from_bytes(data: &[u8]) -> Result<&CompressedAccountPod, ProgramError> {
    bytemuck::try_from_bytes::<CompressedAccountPod>(data)
        .map_err(|_| ProgramError::InvalidAccountData)
}
