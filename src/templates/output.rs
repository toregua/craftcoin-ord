use super::*;

#[derive(Boilerplate)]
pub(crate) struct OutputHtml {
  pub(crate) outpoint: OutPoint,
  pub(crate) list: Option<List>,
  pub(crate) chain: Chain,
  pub(crate) output: TxOut,
  pub(crate) inscriptions: Vec<InscriptionId>,
  pub(crate) cunes: Vec<(SpacedCune, Pile)>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AddressOutputJson {
  pub(crate) outpoint: Vec<OutPoint>,
}

impl AddressOutputJson {
  pub fn new(outputs: Vec<OutPoint>) -> Self {
    Self { outpoint: outputs }
  }
}

impl PageContent for OutputHtml {
  fn title(&self) -> String {
    format!("Output {}", self.outpoint)
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputJson {
  pub address: Option<String>,
  pub inscriptions: Vec<InscriptionId>,
  pub cunes: Vec<(SpacedCune, Pile)>,
  pub script_pubkey: String,
  pub transaction: String,
  pub value: u64,
}

impl OutputJson {
  pub fn new(
    chain: Chain,
    inscriptions: Vec<InscriptionId>,
    outpoint: OutPoint,
    output: TxOut,
    cunes: Vec<(SpacedCune, Pile)>,
  ) -> Self {
    Self {
      address: chain
        .address_from_script(&output.script_pubkey)
        .ok()
        .map(|address| address.to_string()),
      inscriptions,
      cunes,
      script_pubkey: output.script_pubkey.asm(),
      transaction: outpoint.txid.to_string(),
      value: output.value,
    }
  }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    bitcoin::{blockdata::script, PubkeyHash, Script},
  };

  #[test]
  fn unspent_output() {
    assert_regex_match!(
      OutputHtml {
        inscriptions: Vec::new(),
        outpoint: outpoint(1),
        list: Some(List::Unspent(vec![(0, 1), (1, 3)])),
        chain: Chain::Mainnet,
        output: TxOut {
          value: 3,
          script_pubkey: Script::new_p2pkh(&PubkeyHash::all_zeros()),
        },
        cunes: Vec::new(),
      },
      "
        <h1>Output <span class=monospace>1{64}:1</span></h1>
        <dl>
          <dt>value</dt><dd>3</dd>
          <dt>script pubkey</dt><dd class=monospace>OP_DUP OP_HASH160 OP_PUSHBYTES_20 0{40} OP_EQUALVERIFY OP_CHECKSIG</dd>
          <dt>address</dt><dd class=monospace>1111111111111111111114oLvT2</dd>
          <dt>transaction</dt><dd><a class=monospace href=/tx/1{64}>1{64}</a></dd>
        </dl>
        <h2>2 Sat Ranges</h2>
        <ul class=monospace>
          <li><a href=/sat/0 class=mythic>0</a></li>
          <li><a href=/range/1/3 class=common>1–3</a></li>
        </ul>
      "
      .unindent()
    );
  }

  #[test]
  fn spent_output() {
    assert_regex_match!(
      OutputHtml {
        inscriptions: Vec::new(),
        outpoint: outpoint(1),
        list: Some(List::Spent),
        chain: Chain::Mainnet,
        output: TxOut {
          value: 1,
          script_pubkey: script::Builder::new().push_int(0).into_script(),
        },
        cunes: Vec::new(),
      },
      "
        <h1>Output <span class=monospace>1{64}:1</span></h1>
        <dl>
          <dt>value</dt><dd>1</dd>
          <dt>script pubkey</dt><dd class=monospace>OP_0</dd>
          <dt>transaction</dt><dd><a class=monospace href=/tx/1{64}>1{64}</a></dd>
        </dl>
        <p>Output has been spent.</p>
      "
      .unindent()
    );
  }

  #[test]
  fn no_list() {
    assert_regex_match!(
      OutputHtml {
        inscriptions: Vec::new(),
        outpoint: outpoint(1),
        list: None,
        chain: Chain::Mainnet,
        output: TxOut {
          value: 3,
          script_pubkey: Script::new_p2pkh(&PubkeyHash::all_zeros()),
        },
        cunes: Vec::new(),
      }
      .to_string(),
      "
        <h1>Output <span class=monospace>1{64}:1</span></h1>
        <dl>
          <dt>value</dt><dd>3</dd>
          <dt>script pubkey</dt><dd class=monospace>OP_DUP OP_HASH160 OP_PUSHBYTES_20 0{40} OP_EQUALVERIFY OP_CHECKSIG</dd>
          <dt>address</dt><dd class=monospace>1111111111111111111114oLvT2</dd>
          <dt>transaction</dt><dd><a class=monospace href=/tx/1{64}>1{64}</a></dd>
        </dl>
      "
      .unindent()
    );
  }

  #[test]
  fn with_inscriptions() {
    assert_regex_match!(
      OutputHtml {
        inscriptions: vec![inscription_id(1)],
        outpoint: outpoint(1),
        list: None,
        chain: Chain::Mainnet,
        output: TxOut {
          value: 3,
          script_pubkey: Script::new_p2pkh(&PubkeyHash::all_zeros()),
        },
        cunes: Vec::new(),
      },
      "
        <h1>Output <span class=monospace>1{64}:1</span></h1>
        <dl>
          <dt>inscriptions</dt>
          <dd class=thumbnails>
            <a href=/craftscription/1{64}i1><iframe .* src=/preview/1{64}i1></iframe></a>
          </dd>
          .*
        </dl>
      "
      .unindent()
    );
  }

  #[test]
  fn with_cunes() {
    assert_regex_match!(
      OutputHtml {
        inscriptions: Vec::new(),
        outpoint: outpoint(1),
        list: None,
        chain: Chain::Mainnet,
        output: TxOut {
          value: 3,
          script_pubkey: Script::new_p2pkh(&PubkeyHash::all_zeros()),
        },
        cunes: vec![(
          Cune(0),
          Pile {
            amount: 11,
            divisibility: 1,
            symbol: None,
          }
        )],
      },
      "
        <h1>Output <span class=monospace>1{64}:1</span></h1>
        <dl>
          <dt>cunes</dt>
          <dd>
            <table>
              <tr>
                <th>cune</th>
                <th>balance</th>
              </tr>
              <tr>
                <td><a href=/cune/A>A</a></td>
                <td>1.1</td>
              </tr>
            </table>
          </dd>
          .*
        </dl>
      "
      .unindent()
    );
  }
}
