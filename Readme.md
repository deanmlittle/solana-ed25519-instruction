# Ed25519 Signature Readme

## Overview

This Rust module provides a structured way to deserialize and work with Ed25519 signatures in the context of Solana's BPF programs. It allows for deserialization of signature data, extraction of public keys (signers), and retrieval of signed data, making it easier to validate and work with Ed25519 signatures on the Solana blockchain.

## Features

- **Ed25519 Signature Handling**: Provides a structured representation of Ed25519 signature data, including multiple signatures and their associated offsets.
- **Borsh Deserialization**: Implements the `BorshDeserialize` trait for easy deserialization from byte streams, making it compatible with the Borsh serialization format commonly used in Solana programs.
- **Efficient Access**: Provides methods to extract public keys (signers), signatures, and signed data directly from the deserialized data without additional allocations.

## Example Usage

### Deserialize Ed25519Signature from Bytes

```rust
use solana_program::pubkey::Pubkey;
use borsh::BorshDeserialize;

let mut input = /* some byte array containing Ed25519 signature data */;
let signature = Ed25519Signature::deserialize_reader(&mut input).unwrap();
```

### Extract Signers and Signatures

```rust
let signer_pubkey: &Pubkey = signature.0[0].get_signer(&input_data);
let signature_data: &[u8; 64] = signature.0[0].get_signature(&input_data);
```

### Retrieve Signed Data

```rust
let signed_data = signature.0[0].get_data(&input_data);
```

## Details

### Ed25519Signature Structure

The `Ed25519Signature` struct is a wrapper around a `Vec<Ed25519SignatureOffsets>`, where each `Ed25519SignatureOffsets` struct represents a single signature and its associated offsets within the instruction data.

### Borsh Integration

The `Ed25519Signature` and related structs implement the `BorshDeserialize` trait, allowing them to be deserialized from a byte stream using the Borsh serialization format. This makes it easy to work with Solana programs that utilize Borsh for data serialization.

### Methods

- **get_signer(&self, data: &[u8]) -> &Pubkey**: Retrieves the public key (signer) associated with the signature.
- **get_signature(&self, data: &[u8]) -> &[u8; 64]**: Retrieves the signature data.
- **get_data(&self, data: &[u8]) -> Vec<u8>**: Retrieves the signed data.

## Testing

To ensure the correctness of the deserialization and data extraction, it is recommended to write unit tests that validate the behavior of these methods with known inputs.

### Example Test

```rust
#[test]
fn test_signature_extraction() {
    let mut input = /* byte array containing Ed25519 signature data */;
    let signature = Ed25519Signature::deserialize_reader(&mut input).unwrap();
    let signer = signature.0[0].get_signer(&input_data);
    assert_eq!(signer, &expected_pubkey);
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.