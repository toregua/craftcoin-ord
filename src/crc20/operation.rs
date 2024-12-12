use serde_json::{json, Value};

use {
  crate::{Inscription, InscriptionId, SatPoint},
  bitcoin::Txid,
  serde::{Deserialize, Serialize},
};

use crate::crc20::deploy::Deploy;
use crate::crc20::errors::JSONError;
use crate::crc20::mint::Mint;
use crate::crc20::params::PROTOCOL_LITERAL;
use crate::crc20::transfer::Transfer;
use crate::crc20::OperationType;

// collect the inscription operation.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct InscriptionOp {
  pub txid: Txid,
  pub action: Action,
  pub inscription_number: Option<u64>,
  pub inscription_id: InscriptionId,
  pub old_satpoint: SatPoint,
  pub new_satpoint: Option<SatPoint>,
}

// the act of marking an inscription.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Action {
  New { inscription: Inscription },
  Transfer,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
  Deploy(Deploy),
  Mint(Mint),
  InscribeTransfer(Transfer),
  Transfer(Transfer),
}

impl Operation {
  pub fn op_type(&self) -> OperationType {
    match self {
      Operation::Deploy(_) => OperationType::Deploy,
      Operation::Mint(_) => OperationType::Mint,
      Operation::InscribeTransfer(_) => OperationType::InscribeTransfer,
      Operation::Transfer(_) => OperationType::Transfer,
    }
  }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(tag = "op")]
enum RawOperation {
  #[serde(rename = "deploy")]
  Deploy(Deploy),
  #[serde(rename = "mint")]
  Mint(Mint),
  #[serde(rename = "transfer")]
  Transfer(Transfer),
}

pub(crate) fn deserialize_crc20_operation(
  inscription: &Inscription,
  action: &Action,
) -> anyhow::Result<Operation> {
  let content_body = std::str::from_utf8(inscription.body().ok_or(JSONError::InvalidJson)?)?;
  if content_body.len() < 40 {
    return Err(JSONError::NotCRC20Json.into());
  }

  let content_type = inscription
    .content_type()
    .ok_or(JSONError::InvalidContentType)?;

  if !content_type.starts_with("text/plain") && !content_type.starts_with("application/json") {
    return Err(JSONError::UnSupportContentType.into());
  }
  let raw_operation = match deserialize_crc20(content_body) {
    Ok(op) => op,
    Err(e) => {
      return Err(e.into());
    }
  };

  match action {
    Action::New { .. } => match raw_operation {
      RawOperation::Deploy(deploy) => Ok(Operation::Deploy(deploy)),
      RawOperation::Mint(mint) => Ok(Operation::Mint(mint)),
      RawOperation::Transfer(transfer) => Ok(Operation::InscribeTransfer(transfer)),
    },
    Action::Transfer => match raw_operation {
      RawOperation::Transfer(transfer) => Ok(Operation::Transfer(transfer)),
      _ => Err(JSONError::NotCRC20Json.into()),
    },
  }
}

fn deserialize_crc20(s: &str) -> Result<RawOperation, JSONError> {
  let value: Value = serde_json::from_str(s).map_err(|_| JSONError::InvalidJson)?;
  let identifier = value.get("p");
  if let Some(Value::String(p_value)) = identifier {
    if !p_value.eq_ignore_ascii_case(PROTOCOL_LITERAL) {
      return Err(JSONError::NotCRC20Json);
    }
  } else {
    return Err(JSONError::NotCRC20Json);
  }

  serde_json::from_value(value).map_err(|e| JSONError::ParseOperationJsonError(e.to_string()))
}
