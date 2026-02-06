#![allow(unused)]

// src/crypto/crypto.rs
// Phase 1: Implement Core Structs and Macros for Secure Concurrency and Cryptographic Library

use tokio::sync::{Mutex, RwLock, mpsc};
use std::sync::Arc;
use tokio::task;



// Import necessary crates

use secrecy::{SecretBox, ExposeSecret};
use zeroize::Zeroize;

// 
// 
// 
// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, ExprTuple};
// 
// #[proc_macro]
// pub fn attach_error_to_hex(input: TokenStream) -> TokenStream {
//     let args = parse_macro_input!(input as ExprTuple);
// 
//     if args.elems.len() != 2 {
//         return syn::Error::new_spanned(
//             args,
//             "Expected two arguments: hex code and error variant",
//         )
//         .to_compile_error()
//         .into();
//     }
// 
//     let hex_code = &args.elems[0];
//     let error_variant = &args.elems[1];
// 
//     let expanded = quote! {
//         impl From<#error_variant> for sensitive::errors::SecureError {
//             fn from(_: #error_variant) -> Self {
//                 sensitive::errors::SecureError::HexCodeError {
//                     code: #hex_code,
//                     message: stringify!(#error_variant).to_string(),
//                 }
//             }
//         }
//     };
// 
//     TokenStream::from(expanded)
// }
// 



// use sensitive::attach_error_to_hex;
// 
// attach_error_to_hex!(0x1001u32, CryptoError::InvalidKeyLength);















// 
// // Inside pipeline.rs during initialization
// match stage_config.stage_type {
//     crate::config::StageType::Actor => {
//         let crypto = crate::crypto::CryptoBuilder::new()
//             .algorithm(parse_algorithm(&stage_config.algorithm)?)
//             .key_length(parse_key_length(&stage_config.algorithm)?)
//             .with_zeroize()
//             .build()?;
// 
//         let (tx, rx) = tokio::sync::mpsc::channel(32);
//         let actor = Arc::new(crate::concurrencies::actors::EncryptionActor::new(crypto, tx));
//         actor.start(rx).await; // Start the actor's processing task
//         stages.push(actor);
//     }
//     // Handle other stage types similarly
// }
// 
// 







// use aes::Aes256;
// use cipher::{KeyIvInit, StreamCipher};
// 
// // Example encryption in EncryptionActor
// let key = [0u8; 32]; // Obtain key securely
// let iv = [0u8; 16]; // Initialization vector
// let mut cipher = Aes256::new(&key.into(), &iv.into());
// let mut buffer = data.clone();
// cipher.apply_keystream(&mut buffer);
// 
// 
// 
// 
// 
// 