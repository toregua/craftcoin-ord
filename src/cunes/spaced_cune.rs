use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct SpacedCune {
  pub(crate) cune: Cune,
  pub(crate) spacers: u32,
}

impl SpacedCune {
  pub fn new(cune: Cune, spacers: u32) -> Self {
    Self { cune, spacers }
  }
}

impl FromStr for SpacedCune {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut cune = String::new();
    let mut spacers = 0u32;

    for c in s.chars() {
      match c {
        'A'..='Z' => cune.push(c),
        '.' | '•' => {
          let flag = 1 << cune.len().checked_sub(1).context("leading spacer")?;
          if spacers & flag != 0 {
            bail!("double spacer");
          }
          spacers |= flag;
        }
        _ => bail!("invalid character"),
      }
    }

    if 32 - spacers.leading_zeros() >= cune.len().try_into().unwrap() {
      bail!("trailing spacer")
    }

    Ok(SpacedCune {
      cune: cune.parse()?,
      spacers,
    })
  }
}

impl Display for SpacedCune {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let cune = self.cune.to_string();

    for (i, c) in cune.chars().enumerate() {
      write!(f, "{c}")?;

      if i < cune.len() - 1 && self.spacers & 1 << i != 0 {
        write!(f, "•")?;
      }
    }

    Ok(())
  }
}

impl Serialize for SpacedCune {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_str(self)
  }
}

impl<'de> Deserialize<'de> for SpacedCune {
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
  fn display() {
    assert_eq!("A.B".parse::<SpacedCune>().unwrap().to_string(), "A•B");
    assert_eq!("A.B.C".parse::<SpacedCune>().unwrap().to_string(), "A•B•C");
  }

  #[test]
  fn from_str() {
    #[track_caller]
    fn case(s: &str, cune: &str, spacers: u32) {
      assert_eq!(
        s.parse::<SpacedCune>().unwrap(),
        SpacedCune {
          cune: cune.parse().unwrap(),
          spacers
        },
      );
    }

    assert_eq!(
      ".A".parse::<SpacedCune>().unwrap_err().to_string(),
      "leading spacer",
    );

    assert_eq!(
      "A..B".parse::<SpacedCune>().unwrap_err().to_string(),
      "double spacer",
    );

    assert_eq!(
      "A.".parse::<SpacedCune>().unwrap_err().to_string(),
      "trailing spacer",
    );

    assert_eq!(
      "Ax".parse::<SpacedCune>().unwrap_err().to_string(),
      "invalid character",
    );

    case("A.B", "AB", 0b1);
    case("A.B.C", "ABC", 0b11);
    case("A•B", "AB", 0b1);
    case("A•B•C", "ABC", 0b11);
  }

  #[test]
  fn serde() {
    let spaced_cune = SpacedCune {
      cune: Cune(26),
      spacers: 1,
    };
    let json = "\"A•A\"";
    assert_eq!(serde_json::to_string(&spaced_cune).unwrap(), json);
    assert_eq!(
      serde_json::from_str::<SpacedCune>(json).unwrap(),
      spaced_cune
    );
  }
}
