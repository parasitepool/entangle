use bitcoin::{
    Address, Network, OutPoint, Psbt, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness,
    absolute, transaction,
};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A UTXO identified by its outpoint and the output it represents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Utxo {
    /// The outpoint (txid:vout) identifying this UTXO.
    pub outpoint: OutPoint,
    /// The transaction output (value + scriptPubKey).
    pub txout: TxOut,
}

/// A request to build a swap PSBT.
///
/// Party A's UTXO is always required. Party B's UTXO is optional —
/// if absent, the PSBT spends only A's UTXO to B's address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapRequest {
    /// The UTXO owned by party A.
    pub utxo_a: Utxo,
    /// The address where party A wants to receive funds.
    pub address_a: Address<bitcoin::address::NetworkUnchecked>,
    /// The UTXO owned by party B (optional).
    pub utxo_b: Option<Utxo>,
    /// The address where party B wants to receive funds.
    pub address_b: Address<bitcoin::address::NetworkUnchecked>,
    /// The bitcoin network to validate addresses against.
    pub network: Network,
}

/// Errors that can occur when building a swap PSBT.
#[derive(Debug)]
pub enum SwapError {
    /// An address does not belong to the expected network.
    NetworkMismatch { expected: Network, address: String },
    /// The unsigned transaction could not be converted to a PSBT.
    PsbtCreation(String),
}

impl fmt::Display for SwapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwapError::NetworkMismatch { expected, address } => {
                write!(f, "address {address} does not match network {expected}")
            }
            SwapError::PsbtCreation(msg) => write!(f, "failed to create PSBT: {msg}"),
        }
    }
}

impl std::error::Error for SwapError {}

/// Validate an unchecked address against the expected network.
fn validate_address(
    address: &Address<bitcoin::address::NetworkUnchecked>,
    network: Network,
) -> Result<Address, SwapError> {
    let addr_string = address.clone().assume_checked().to_string();
    address
        .clone()
        .require_network(network)
        .map_err(|_| SwapError::NetworkMismatch {
            expected: network,
            address: addr_string,
        })
}

/// Build a PSBT that swaps UTXOs between two parties.
///
/// The resulting PSBT contains:
/// - An input spending `utxo_a` with an output paying to `address_b`
/// - If `utxo_b` is provided: an input spending `utxo_b` with an output paying to `address_a`
///
/// The PSBT is unsigned. Party A signs first, then party B.
pub fn build_swap_psbt(request: &SwapRequest) -> Result<Psbt, SwapError> {
    let address_a = validate_address(&request.address_a, request.network)?;
    let address_b = validate_address(&request.address_b, request.network)?;

    // Build inputs
    let mut inputs = vec![TxIn {
        previous_output: request.utxo_a.outpoint,
        script_sig: ScriptBuf::new(),
        sequence: Sequence::MAX,
        witness: Witness::default(),
    }];

    if let Some(ref utxo_b) = request.utxo_b {
        inputs.push(TxIn {
            previous_output: utxo_b.outpoint,
            script_sig: ScriptBuf::new(),
            sequence: Sequence::MAX,
            witness: Witness::default(),
        });
    }

    // Build outputs: A's value → B's address, B's value → A's address
    let mut outputs = vec![TxOut {
        value: request.utxo_a.txout.value,
        script_pubkey: address_b.script_pubkey(),
    }];

    if let Some(ref utxo_b) = request.utxo_b {
        outputs.push(TxOut {
            value: utxo_b.txout.value,
            script_pubkey: address_a.script_pubkey(),
        });
    }

    let tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input: inputs,
        output: outputs,
    };

    let mut psbt =
        Psbt::from_unsigned_tx(tx).map_err(|e| SwapError::PsbtCreation(e.to_string()))?;

    // Set witness_utxo on PSBT inputs for signing support
    psbt.inputs[0].witness_utxo = Some(request.utxo_a.txout.clone());
    if let Some(ref utxo_b) = request.utxo_b {
        psbt.inputs[1].witness_utxo = Some(utxo_b.txout.clone());
    }

    Ok(psbt)
}
