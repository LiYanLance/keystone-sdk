// use wasm_bindgen::prelude::*;
use ur_registry::solana::sol_signature::SolSignature;
use std::str;
use hex;
use ur_registry::traits::{From, UR};
use std::ffi::{CStr};
use std::io::Bytes;
use std::os::raw::{c_char};
use ffi_support::{rust_string_to_c};
use std::string::String;
use crate::response::{PtrResponse, Response};
use crate::types::PtrVoid;

use crate::export;

// #[wasm_bindgen]
// pub fn parse_sol_signature (
//     cbor_hex: String
// ) -> String {
//     let cbor = hex::decode(cbor_hex).unwrap();
//     let res = serde_cbor::from_slice(cbor.as_slice()).unwrap();
//     let sol_signature = SolSignature::from_cbor(res).unwrap();
//     let sig = sol_signature.get_signature();
//     hex::encode(sig)
// }


export! {
    @Java_com_keystone_sdk_KeystoneSDK_parseSolSignature
	fn parse_sol_signature(
		cbor_hex: &str
	) -> String {
        let cbor = hex::decode(cbor_hex.to_string()).unwrap();
        let res = serde_cbor::from_slice(cbor.as_slice()).unwrap();
        let sol_signature = SolSignature::from_cbor(res).unwrap();
        hex::encode(sol_signature.get_signature())
	}
}

// fn cbor_to_sig(cbor_str: String) -> SolSignature {
//     let cbor = hex::decode(cbor_str).unwrap();
//     let res = serde_cbor::from_slice(cbor.as_slice()).unwrap();
//     SolSignature::from_cbor(res).unwrap()
// }
//
// #[no_mangle]
// extern "C" fn parse_sol_signature(cbor_hex: *const c_char) -> *const c_char {
//     let cbor_str=  match unsafe { CStr::from_ptr(cbor_hex) }.to_str() {
//         Ok(v) => v.to_string(),
//         Err(_) => String::from("")
//     };
//
//     let sig = parse_sig_from_cbor(cbor_str);
//     rust_string_to_c(hex::encode(sig))
// }

// #[no_mangle]
// pub extern "C" fn parse_signature_ur(cbor: String) -> PtrResponse {
//     let sol_sig = cbor_to_sig(cbor);
//     Response::success_object(Box::into_raw(Box::new(sol_sig)) as PtrVoid).c_ptr()
// }

//
// export! {
//     @Java_io_parity_substrateSign_SubstrateSignModule_ethkeyBrainwalletAddress
//     fn parse_sol_signature(cbor_hex: *const c_char) -> *const c_char {
//         let cbor_str=  match unsafe { CStr::from_ptr(cbor_hex) }.to_str() {
//             Ok(v) => v.to_string(),
//             Err(_) => String::from("")
//         };
//
//         let cbor = hex::decode(cbor_str).unwrap();
//         let res = serde_cbor::from_slice(cbor.as_slice()).unwrap();
//         let sol_signature = SolSignature::from_cbor(res).unwrap();
//         let sig = sol_signature.get_signature();
//         rust_string_to_c(hex::encode(sig))
//     }
// }



#[cfg(test)]
mod tests {
    use super::*;
    //
    // #[test]
    // fn test_encode() {
    //     assert_eq!(
    //         "d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f7",
    //         parse_sol_signature("a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025840d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f7".to_string())
    //     );
    // }

    #[test]
    fn test_parse() {
        assert_eq!(
            "d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f7",
            parse_sol_signature("a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025840d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f7")
        );
    }
}
