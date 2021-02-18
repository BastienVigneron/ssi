use crate::error::Error;

use crate::jwk::{Params, JWK};

const TZ1_EDPK: [u8; 4] = [0x65, 0x64, 0x70, 0x6b];
const TZ2_SPPK: [u8; 4] = [0x73, 0x70, 0x70, 0x6b];
const TZ3_P2PK: [u8; 4] = [0x70, 0x32, 0x70, 0x6b];

const TZ1_HASH: [u8; 3] = [0x06, 0xa1, 0x9f];
const TZ2_HASH: [u8; 3] = [0x06, 0xa1, 0xa1];
const TZ3_HASH: [u8; 3] = [0x06, 0xa1, 0xa4];

// 'E' + 'd25519BLAKE2BDigestSize20Base58CheckEncoded'.length + 'Sig2021'

fn curve_to_prefixes(curve: &str) -> Result<(&'static [u8; 4], &'static [u8; 3]), Error> {
    let prefix = match curve {
        "Ed25519" => (&TZ1_EDPK, &TZ1_HASH),
        "secp256k1" => (&TZ2_SPPK, &TZ2_HASH),
        "P-256" => (&TZ3_P2PK, &TZ3_HASH),
        _ => return Err(Error::KeyTypeNotImplemented),
    };
    Ok(prefix)
}

pub fn hash_public_key(jwk: &JWK) -> Result<String, Error> {
    let params = match jwk.params {
        Params::OKP(ref okp_params) => okp_params,
        _ => return Err(Error::KeyTypeNotImplemented),
    };
    let (inner_prefix, outer_prefix) = curve_to_prefixes(&params.curve)?;
    let encoded = bs58::encode(&params.public_key.0);
    let pk_b58_vec = encoded.into_vec();
    let mut inner = Vec::with_capacity(4 + pk_b58_vec.len());
    inner.extend_from_slice(inner_prefix);
    inner.extend(pk_b58_vec);
    let mut hasher = blake2b_simd::Params::new();
    hasher.hash_length(20);
    let blake2b = hasher.hash(&inner);
    let blake2b = blake2b.as_bytes();
    let mut outer = Vec::with_capacity(23);
    outer.extend_from_slice(outer_prefix);
    outer.extend_from_slice(&blake2b);
    let encoded = bs58::encode(&outer).with_check().into_string();
    Ok(encoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hash() {
        let jwk: JWK = serde_json::from_str(
            r#"{
              "crv": "Ed25519",
              "kty": "OKP",
              "x": "G80iskrv_nE69qbGLSpeOHJgmV4MKIzsy5l5iT6pCww"
            }"#,
        )
        .unwrap();
        let hash = hash_public_key(&jwk).unwrap();
        assert_eq!(hash, "tz1iY7Am8EqrewptzQXYRZDPKvYnFLzWRgBK");
    }
}