# Auditor Retrieval Process Guide

## Overview

This document provides third-party auditors with comprehensive instructions for retrieving and verifying encrypted datasets stored on IPFS/Filecoin through the Stellar Privacy Analytics platform.

## Prerequisites

- Access to the Stellar Privacy Analytics API
- Valid authentication credentials
- Dataset CID (Content Identifier)
- Decryption key or key hash (if applicable)

## Retrieval Process

### 1. Data Availability Verification

Before attempting retrieval, verify data availability:

```bash
GET /api/ipfs/availability/enhanced/{cid}
```

**Response:**
```json
{
  "success": true,
  "availability": {
    "available": true,
    "pinned": true,
    "filecoinDeals": [
      {
        "dealId": 12345,
        "status": "Active",
        "pieceCid": "bafy...",
        "provider": "f01234",
        "startEpoch": 123456,
        "endEpoch": 654321
      }
    ],
    "pinCount": 1,
    "lastChecked": 1703020800000,
    "gatewayAccessible": true
  },
  "gatewayUrl": "https://gateway.pinata.cloud/ipfs/bafy..."
}
```

### 2. CID Immutability Verification

Check if the CID is immutable (locked for analysis):

```bash
# Query the Stellar smart contract
# Function: is_cid_immutable(env, cid) -> Result<bool, Error>
```

### 3. Dataset Metadata Retrieval

Get dataset information from the Stellar contract:

```bash
# Function: get_dataset(env, cid) -> Result<IPFSDataset, Error>
```

**Expected Structure:**
```rust
pub struct IPFSDataset {
    pub cid: String,
    pub dataset_hash: BytesN<32>,
    pub uploader: Address,
    pub timestamp: u64,
    pub size_bytes: u64,
    pub encrypted: bool,
    pub version: u32,
    pub pinned: bool,
    pub decryption_key_hash: Option<BytesN<32>>,
}
```

### 4. Encrypted Data Retrieval

For encrypted datasets, use the dedicated endpoint:

```bash
GET /api/ipfs/encrypted/download/{cid}?encryptionKeyId={keyId}&validateIntegrity=true
```

**Response:**
```json
{
  "success": true,
  "data": {
    "data": "base64-encoded-encrypted-data",
    "integrity": {
      "verified": true,
      "expectedHash": "sha256-hash",
      "actualHash": "sha256-hash"
    },
    "metadata": {
      "cid": "bafy...",
      "size": 1024000,
      "hash": "sha256-hash",
      "timestamp": "2024-01-01T00:00:00.000Z",
      "version": 1,
      "keyId": "key_1703020800000"
    }
  },
  "message": "Data downloaded and decrypted successfully"
}
```

### 5. Direct IPFS Retrieval (Unencrypted Data)

For unencrypted datasets:

```bash
GET /api/ipfs/retrieve/{cid}
```

### 6. Streaming Retrieval (Large Files)

For large datasets, use streaming:

```bash
GET /api/ipfs/encrypted/stream/{cid}?encryptionKeyId={keyId}&chunkSize=1048576
```

## Decryption Process

### 1. Key Verification

Verify the decryption key against the stored hash:

```bash
POST /api/ipfs/verify-key
Content-Type: application/json

{
  "decryptionKey": "your-decryption-key",
  "keyHash": "stored-key-hash"
}
```

### 2. Local Decryption

If you have the raw encrypted data and key:

```javascript
import crypto from 'crypto';

function decryptData(encryptedData, key) {
  // Extract IV, authTag, and encrypted payload
  const iv = encryptedData.slice(0, 16);
  const authTag = encryptedData.slice(16, 32);
  const encryptedPayload = encryptedData.slice(32);
  
  const decipher = crypto.createDecipherGCM('aes-256-gcm', key);
  decipher.setIV(iv);
  decipher.setAuthTag(authTag);
  
  return Buffer.concat([
    decipher.update(encryptedPayload),
    decipher.final()
  ]);
}
```

## Verification Steps

### 1. Data Integrity Verification

Verify SHA-256 hash matches the stored hash:

```bash
GET /api/ipfs/encrypted/integrity/verify/{cid}?expectedHash={sha256}
```

### 2. Blockchain Verification

Verify the CID is recorded on the Stellar blockchain:

```rust
// Query the smart contract
let dataset = contract.get_dataset(env, cid)?;
assert!(dataset.pinned);
```

### 3. Version History

Check dataset versioning for historical analysis:

```bash
GET /api/ipfs/encrypted/versions/{datasetId}
```

## Security Considerations

### 1. Key Management

- Never store decryption keys with the encrypted data
- Use secure key storage solutions
- Rotate keys regularly
- Verify key hashes before use

### 2. Data Validation

- Always verify data integrity hashes
- Check CID immutability status
- Validate blockchain records
- Confirm Filecoin deal status

### 3. Access Control

- Use proper authentication
- Validate authorization for sensitive datasets
- Log all access attempts
- Implement rate limiting

## Troubleshooting

### Common Issues

1. **CID Not Available**
   - Check pinning status
   - Verify Filecoin deals
   - Test gateway accessibility

2. **Decryption Failures**
   - Verify key correctness
   - Check key hash matching
   - Validate data integrity

3. **Version Conflicts**
   - Check dataset versioning
   - Verify historical versions
   - Confirm version immutability

### Error Codes

| Error | Description | Solution |
|-------|-------------|----------|
| DataNotAvailable | CID not pinned or accessible | Check availability status |
| InvalidDecryptionKey | Key verification failed | Verify key and hash |
| VersionMismatch | Dataset version conflict | Check version history |
| CIDImmutable | Cannot modify immutable CID | Use correct version |

## API Reference

### Data Availability

- `GET /api/ipfs/availability/{cid}` - Basic availability check
- `GET /api/ipfs/availability/enhanced/{cid}` - Enhanced check with Filecoin deals

### Data Retrieval

- `GET /api/ipfs/retrieve/{cid}` - Direct retrieval
- `GET /api/ipfs/encrypted/download/{cid}` - Encrypted retrieval
- `GET /api/ipfs/encrypted/stream/{cid}` - Streaming retrieval

### Verification

- `POST /api/ipfs/verify-key` - Key verification
- `GET /api/ipfs/encrypted/integrity/verify/{cid}` - Integrity verification

### Metadata

- `GET /api/ipfs/encrypted/versions/{datasetId}` - Version history
- `GET /api/ipfs/deals/{cid}` - Filecoin deal information

## Compliance and Auditing

### Audit Trail

All operations are logged and recorded:

1. **Blockchain Records**: Immutable CID storage on Stellar
2. **Access Logs**: API access and retrieval attempts
3. **Integrity Logs**: Hash verification results
4. **Key Logs**: Key usage and verification attempts

### Regulatory Compliance

The system supports:

- **GDPR**: Right to access and rectification
- **CCPA**: Consumer data access rights
- **HIPAA**: Healthcare data protection (when applicable)
- **SOX**: Financial data integrity requirements

### Data Retention

- Automatic cleanup based on retention policies
- Version history maintenance
- Audit log preservation
- Compliance reporting

## Contact and Support

For auditor support and questions:

- **Technical Support**: support@stellar-ecosystem.com
- **Security Issues**: security@stellar-ecosystem.com
- **Compliance Questions**: compliance@stellar-ecosystem.com

## Appendix

### A. Sample Scripts

#### Python Retrieval Script
```python
import requests
import base64
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

def retrieve_encrypted_dataset(cid, encryption_key, api_base_url):
    # Get encrypted data
    response = requests.get(f"{api_base_url}/api/ipfs/encrypted/download/{cid}", 
                           params={"encryptionKeyId": encryption_key})
    data = response.json()
    
    # Decode and decrypt
    encrypted_data = base64.b64decode(data['data']['data'])
    
    # Extract components and decrypt (implementation specific)
    # ...
    
    return decrypted_data
```

#### Node.js Verification Script
```javascript
const crypto = require('crypto');
const axios = require('axios');

async function verifyDatasetIntegrity(cid, expectedHash, apiBaseUrl) {
  const response = await axios.get(
    `${apiBaseUrl}/api/ipfs/encrypted/integrity/verify/${cid}`,
    { params: { expectedHash } }
  );
  
  return response.data.data.isValid;
}
```

### B. Configuration

#### Environment Variables
```env
IPFS_GATEWAY_URL=https://gateway.pinata.cloud
IPFS_PUBLIC_GATEWAY=https://gateway.pinata.cloud
PINATA_API_KEY=your-api-key
PINATA_SECRET_KEY=your-secret-key
IPFS_TIMEOUT=30000
```

This guide provides auditors with all necessary information to securely retrieve, verify, and validate datasets stored in the Stellar Privacy Analytics ecosystem.
