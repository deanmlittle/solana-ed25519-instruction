#[cfg(feature = "anchor")]
use anchor_lang::prelude::*;
#[cfg(not(feature = "anchor"))]
use solana_program::pubkey::Pubkey;
use borsh::BorshDeserialize;


#[derive(Debug)]
pub struct Ed25519Signature(pub Vec<Ed25519SignatureOffsets>);

impl BorshDeserialize for Ed25519Signature {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::result::Result<Ed25519Signature, std::io::Error> {
        let header = Ed25519SignatureHeader::deserialize_reader(reader)?;
        let mut offsets: Vec<Ed25519SignatureOffsets> = Vec::with_capacity(header.num_signatures as usize);
        for _ in 0..header.num_signatures {
            offsets.push(Ed25519SignatureOffsets::deserialize_reader(reader)?);
        }
        Ok(Self(offsets))
    }
}

#[derive(BorshDeserialize, Debug)]
pub struct Ed25519SignatureHeader {
    pub num_signatures: u8,
    pub padding: u8,
}

#[derive(BorshDeserialize, Debug)]
pub struct Ed25519SignatureOffsets {
    pub signature_offset: u16,
    pub signature_instruction_index: u16,
    pub public_key_offset: u16,
    pub public_key_instruction_index: u16,
    pub message_data_offset: u16,
    pub message_data_size: u16,
    pub message_instruction_index: u16
}

impl Ed25519SignatureOffsets {
    pub fn get_signer(&self, data: &[u8]) -> &Pubkey {
        let slice = &data[self.public_key_offset as usize..self.public_key_offset as usize + 32];
        unsafe { &*(slice.as_ptr() as *const Pubkey) }
    }

    pub fn get_signature(&self, data: &[u8]) -> &[u8; 64] {
        let slice = &data[self.signature_offset as usize..self.signature_offset as usize + 64];
        unsafe { &*(slice.as_ptr() as *const [u8; 64]) }
    }

    pub fn get_data(&self, data: &[u8]) -> Vec<u8> {
        data[self.message_data_offset as usize..self.message_data_offset as usize + self.message_data_size as usize].to_vec()
    }
}