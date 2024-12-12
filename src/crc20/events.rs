use super::*;
use crate::crc20::script_key::ScriptKey;
use crate::{InscriptionId, SatPoint};
use bitcoin::Txid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum OperationType {
  Deploy,
  Mint,
  InscribeTransfer,
  Transfer,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Receipt {
  pub inscription_id: InscriptionId,
  pub inscription_number: i64,
  pub old_satpoint: SatPoint,
  pub new_satpoint: SatPoint,
  pub op: OperationType,
  pub from: ScriptKey,
  pub to: ScriptKey,
  pub result: Result<Event, CRC20Error>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Event {
  Deploy(DeployEvent),
  Mint(MintEvent),
  InscribeTransfer(InscribeTransferEvent),
  Transfer(TransferEvent),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DeployEvent {
  pub txid: Option<Txid>,
  pub vout: u32,
  pub deployed_by: ScriptKey,
  pub supply: u128,
  pub limit_per_mint: u128,
  pub decimal: u8,
  pub tick: Tick,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MintEvent {
  pub txid: Option<Txid>,
  pub vout: u32,
  pub to: ScriptKey,
  pub tick: Tick,
  pub amount: u128,
  pub msg: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InscribeTransferEvent {
  pub txid: Option<Txid>,
  pub to: ScriptKey,
  pub vout: u32,
  pub tick: Tick,
  pub amount: u128,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TransferEvent {
  pub txid: Option<Txid>,
  pub from: ScriptKey,
  pub to: ScriptKey,
  pub vout: u32,
  pub tick: Tick,
  pub amount: u128,
}
