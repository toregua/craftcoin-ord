use super::*;

#[derive(Default, Serialize, Debug, PartialEq, Copy, Clone)]
pub struct Etching {
  pub divisibility: Option<u8>,
  pub terms: Option<Terms>,
  pub premine: Option<u128>,
  pub cune: Option<Cune>,
  pub spacers: Option<u32>,
  pub symbol: Option<char>,
  pub turbo: bool,
}
