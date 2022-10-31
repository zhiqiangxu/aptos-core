// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use aptos_crypto::ValidCryptoMaterialStringExt;
use aptos_keygen::KeyGen;
use aptos_types::transaction::authenticator::AuthenticationKey;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Parser, Default)]
struct Arguments {
    #[clap(short, long)]
    pub n: i32,
}

#[derive(Serialize, Deserialize)]
struct Account {
    pub addr: String,
    pub pk: String,
}
fn main() {
    let mut keygen = KeyGen::from_os_rng();
    let args = Arguments::parse();

    let mut results = vec![];
    let mut m = HashMap::new();
    for _ in 0..args.n {
        let (privkey, pubkey) = keygen.generate_ed25519_keypair();

        let auth_key = AuthenticationKey::ed25519(&pubkey);
        let account_addr = auth_key.derived_address();

        if m.contains_key(&account_addr) {
            println!("oops, duplicate key founded!!!");
            continue;
        }
        m.insert(account_addr, ());

        results.push(Account {
            pk: privkey.to_encoded_string().unwrap(),
            addr: account_addr.to_hex(),
        })
    }

    println!("{}", serde_json::to_string(&results).unwrap())
}
