use {super::*, std::num::TryFromIntError};

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq, Ord, PartialOrd)]
pub struct CuneId {
  pub height: u64,
  pub index: u32,
}

impl TryFrom<u128> for CuneId {
  type Error = TryFromIntError;

  fn try_from(n: u128) -> Result<Self, Self::Error> {
    Ok(Self {
      height: u64::try_from(n >> 16)?,
      index: u32::try_from(n & 0xFFFF).unwrap(),
    })
  }
}

impl From<CuneId> for u128 {
  fn from(id: CuneId) -> Self {
    u128::from(id.height) << 16 | u128::from(id.index)
  }
}

impl Display for CuneId {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}:{}", self.height, self.index,)
  }
}

impl FromStr for CuneId {
  type Err = crate::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (height, index) = s
      .split_once(':')
      .ok_or_else(|| anyhow!("invalid cune ID: {s}"))?;

    Ok(Self {
      height: height.parse()?,
      index: index.parse()?,
    })
  }
}

impl Serialize for CuneId {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_str(self)
  }
}

impl<'de> Deserialize<'de> for CuneId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    Ok(DeserializeFromStr::deserialize(deserializer)?.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn cune_id_to_128() {
    assert_eq!(
      0b11_0000_0000_0000_0001u128,
      CuneId {
        height: 3,
        index: 1,
      }
      .into()
    );
  }

  #[test]
  fn display() {
    assert_eq!(
      CuneId {
        height: 1,
        index: 2
      }
      .to_string(),
      "1:2"
    );
  }

  #[test]
  fn from_str() {
    assert!(":".parse::<CuneId>().is_err());
    assert!("1:".parse::<CuneId>().is_err());
    assert!(":2".parse::<CuneId>().is_err());
    assert!("a:2".parse::<CuneId>().is_err());
    assert!("1:a".parse::<CuneId>().is_err());
    assert_eq!(
      "1:2".parse::<CuneId>().unwrap(),
      CuneId {
        height: 1,
        index: 2
      }
    );
  }

  #[test]
  fn try_from() {
    assert_eq!(
      CuneId::try_from(0x060504030201).unwrap(),
      CuneId {
        height: 0x06050403,
        index: 0x0201
      }
    );

    assert!(CuneId::try_from(0x07060504030201).is_err());
  }

  #[test]
  fn serde() {
    let cune_id = CuneId {
      height: 1,
      index: 2,
    };
    let json = "\"1:2\"";
    assert_eq!(serde_json::to_string(&cune_id).unwrap(), json);
    assert_eq!(serde_json::from_str::<CuneId>(json).unwrap(), cune_id);
  }
}
