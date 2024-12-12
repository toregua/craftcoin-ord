use super::*;

#[derive(Boilerplate, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct CuneHtml {
  pub(crate) entry: CuneEntry,
  pub(crate) id: CuneId,
  pub(crate) mintable: bool,
  pub(crate) inscription: Option<InscriptionId>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct CuneEntryJson {
  pub(crate) burned: u128,
  pub(crate) divisibility: u8,
  pub(crate) etching: Txid,
  pub(crate) mint: Option<Terms>,
  pub(crate) mints: u128,
  pub(crate) number: u64,
  pub(crate) cune: SpacedCune,
  pub(crate) supply: u128,
  pub(crate) symbol: Option<char>,
  pub(crate) timestamp: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct CuneJson {
  pub(crate) entry: CuneEntryJson,
  pub(crate) id: CuneId,
  pub(crate) mintable: bool,
  pub(crate) inscription: Option<InscriptionId>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct CuneOutputJson {
  pub(crate) cune: SpacedCune,
  pub(crate) balances: Pile,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct CuneAddressJson {
  pub(crate) cunes: Vec<CuneBalance>,
  pub(crate) total_cunes: usize,
  pub(crate) total_elements: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CuneBalance {
  pub(crate) cune: SpacedCune,
  pub(crate) divisibility: u8,
  pub(crate) symbol: Option<char>,
  pub(crate) total_balance: u128,
  pub(crate) total_outputs: u128,
  pub(crate) balances: Vec<CuneOutput>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CuneOutput {
  pub(crate) txid: Txid,
  pub(crate) vout: u32,
  pub(crate) script: Script,
  pub(crate) shibes: u64,
  pub(crate) balance: u128,
}

impl PageContent for CuneHtml {
  fn title(&self) -> String {
    format!("Cune {}", self.entry.spaced_cune())
  }
}

#[cfg(test)]
mod tests {
  use {super::*, crate::cunes::Cune};

  #[test]
  fn display() {
    assert_regex_match!(
      CuneHtml {
        entry: CuneEntry {
          burned: 123456789123456789,
          divisibility: 9,
          etching: Txid::all_zeros(),
          number: 25,
          cune: Cune(u128::max_value()),
          supply: 123456789123456789,
          symbol: Some('$'),
          mint: Some(MintEntry {
            end: Some(11),
            limit: Some(1000000001),
            deadline: Some(7),
          }),
          end: Some(11),
          timestamp: 0,
        },
        id: CuneId {
          height: 10,
          index: 9,
        },
        inscription: Some(InscriptionId {
          txid: Txid::all_zeros(),
          index: 0,
        }),
      },
      r"<h1>BCGDENLQRQWDSLRUGSNLBTMFIJAV</h1>
<iframe .* src=/preview/0{64}i0></iframe>
<dl>
  <dt>id</dt>
  <dd>10:9</dd>
  <dt>number</dt>
  <dd>25</dd>
  <dt>supply</dt>
  <dd>\$123456789.123456789</dd>
  <dt>burned</dt>
  <dd>\$123456789.123456789</dd>
  <dt>divisibility</dt>
  <dd>9</dd>
  <dt>open etching end</dt>
  <dd><a href=/block/11>11</a></dd>
  <dt>open etching limit</dt>
  <dd>\$1.000000001</dd>
  <dt>symbol</dt>
  <dd>\$</dd>
  <dt>etching</dt>
  <dd><a class=monospace href=/tx/0{64}>0{64}</a></dd>
  <dt>inscription</dt>
  <dd><a class=monospace href=/inscription/0{64}i0>0{64}i0</a></dd>
</dl>
"
    );
  }
}
