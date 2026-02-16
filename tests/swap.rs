use bitcoin::Network;

use entangle::swap::{SwapRequest, build_swap_psbt};

use crate::*;

#[test]
fn two_sided_swap_produces_valid_psbt() {
    let request = make_swap_request(true);
    let psbt = build_swap_psbt(&request).unwrap();

    assert_eq!(psbt.unsigned_tx.input.len(), 2);
    assert_eq!(psbt.unsigned_tx.output.len(), 2);

    // Inputs reference the correct outpoints
    assert_eq!(
        psbt.unsigned_tx.input[0].previous_output,
        request.utxo_a.outpoint
    );
    assert_eq!(
        psbt.unsigned_tx.input[1].previous_output,
        request.utxo_b.as_ref().unwrap().outpoint
    );

    // Output 0: A's value → B's address
    assert_eq!(psbt.unsigned_tx.output[0].value, request.utxo_a.txout.value);
    let addr_b = request
        .address_b
        .clone()
        .require_network(request.network)
        .unwrap();
    assert_eq!(
        psbt.unsigned_tx.output[0].script_pubkey,
        addr_b.script_pubkey()
    );

    // Output 1: B's value → A's address
    assert_eq!(
        psbt.unsigned_tx.output[1].value,
        request.utxo_b.as_ref().unwrap().txout.value
    );
    let addr_a = request
        .address_a
        .clone()
        .require_network(request.network)
        .unwrap();
    assert_eq!(
        psbt.unsigned_tx.output[1].script_pubkey,
        addr_a.script_pubkey()
    );

    // Witness UTXOs set for signing
    assert_eq!(
        psbt.inputs[0].witness_utxo,
        Some(request.utxo_a.txout.clone())
    );
    assert_eq!(
        psbt.inputs[1].witness_utxo,
        Some(request.utxo_b.as_ref().unwrap().txout.clone())
    );
}

#[test]
fn one_sided_swap_produces_valid_psbt() {
    let request = make_swap_request(false);
    let psbt = build_swap_psbt(&request).unwrap();

    assert_eq!(psbt.unsigned_tx.input.len(), 1);
    assert_eq!(psbt.unsigned_tx.output.len(), 1);
    assert_eq!(
        psbt.unsigned_tx.input[0].previous_output,
        request.utxo_a.outpoint
    );
    assert_eq!(psbt.unsigned_tx.output[0].value, request.utxo_a.txout.value);
    assert_eq!(
        psbt.inputs[0].witness_utxo,
        Some(request.utxo_a.txout.clone())
    );
}

#[test]
fn network_mismatch_address_a() {
    let request = SwapRequest {
        utxo_a: make_utxo(50_000, 0),
        address_a: MAINNET_ADDR.parse().unwrap(),
        utxo_b: None,
        address_b: TESTNET_ADDR_B.parse().unwrap(),
        network: Network::Testnet,
    };
    let err = build_swap_psbt(&request).unwrap_err();
    assert!(matches!(
        err,
        entangle::swap::SwapError::NetworkMismatch { .. }
    ));
}

#[test]
fn network_mismatch_address_b() {
    let request = SwapRequest {
        utxo_a: make_utxo(50_000, 0),
        address_a: TESTNET_ADDR_A.parse().unwrap(),
        utxo_b: None,
        address_b: MAINNET_ADDR.parse().unwrap(),
        network: Network::Testnet,
    };
    let err = build_swap_psbt(&request).unwrap_err();
    assert!(matches!(
        err,
        entangle::swap::SwapError::NetworkMismatch { .. }
    ));
}

#[test]
fn psbt_base64_round_trips() {
    let request = make_swap_request(true);
    let psbt = build_swap_psbt(&request).unwrap();
    let base64_str = psbt.to_string();
    let parsed: bitcoin::Psbt = base64_str.parse().unwrap();
    assert_eq!(parsed.unsigned_tx.input.len(), 2);
    assert_eq!(parsed.unsigned_tx.output.len(), 2);
}
