mod api;
mod listing;
mod server;
mod swap;

use std::str::FromStr;

use bitcoin::{Amount, Network, OutPoint, ScriptBuf, TxOut};

use entangle::swap::{SwapRequest, Utxo};

pub const TESTNET_ADDR_A: &str = "tb1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q0sl5k7";
pub const TESTNET_ADDR_B: &str = "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx";
pub const MAINNET_ADDR: &str = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";

pub fn make_utxo(sats: u64, vout: u32) -> Utxo {
    let outpoint = OutPoint::from_str(&format!(
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa:{vout}"
    ))
    .unwrap();
    Utxo {
        outpoint,
        txout: TxOut {
            value: Amount::from_sat(sats),
            script_pubkey: ScriptBuf::new(),
        },
    }
}

pub fn make_swap_request(two_sided: bool) -> SwapRequest {
    SwapRequest {
        utxo_a: make_utxo(50_000, 0),
        address_a: TESTNET_ADDR_A.parse().unwrap(),
        utxo_b: if two_sided {
            Some(make_utxo(75_000, 1))
        } else {
            None
        },
        address_b: TESTNET_ADDR_B.parse().unwrap(),
        network: Network::Testnet,
    }
}

pub fn swap_request_json(two_sided: bool) -> String {
    serde_json::to_string(&make_swap_request(two_sided)).unwrap()
}

pub fn mismatch_request_json() -> String {
    let request = SwapRequest {
        utxo_a: make_utxo(50_000, 0),
        address_a: MAINNET_ADDR.parse().unwrap(),
        utxo_b: None,
        address_b: TESTNET_ADDR_B.parse().unwrap(),
        network: Network::Testnet,
    };
    serde_json::to_string(&request).unwrap()
}
