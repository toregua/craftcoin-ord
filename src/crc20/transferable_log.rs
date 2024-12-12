use super::*;
use crate::crc20::script_key::ScriptKey;
use crate::InscriptionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransferableLog {
  pub inscription_id: InscriptionId,
  pub inscription_number: u64,
  pub amount: u128,
  pub tick: Tick,
  pub owner: ScriptKey,
}
