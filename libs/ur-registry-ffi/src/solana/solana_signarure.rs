use wasm_bindgen::prelude::*;
use ur_registry::solana::sol_signature::SolSignature;
use std::str;
use hex;
use ur_registry::traits::{From, UR};

#[wasm_bindgen]
pub fn parse_sol_signature (
    cbor_hex: String
) -> String {
    let cbor = hex::decode(cbor_hex).unwrap();
    let res = serde_cbor::from_slice(cbor.as_slice()).unwrap();
    let sol_signature = SolSignature::from_cbor(res).unwrap();
    let sig = sol_signature.get_signature();
    hex::encode(sig)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(
            "d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f7",
            parse_sol_signature("a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025840d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f7".to_string())
        );
    }
}
