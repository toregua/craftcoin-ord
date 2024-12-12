use super::*;
use crate::sat::Sat;
use crate::sat_point::SatPoint;

#[derive(Boilerplate, Default)]
pub(crate) struct InscriptionHtml {
  pub(crate) chain: Chain,
  pub(crate) genesis_fee: u64,
  pub(crate) genesis_height: u32,
  pub(crate) inscription: Inscription,
  pub(crate) inscription_id: InscriptionId,
  pub(crate) inscription_number: u64,
  pub(crate) next: Option<InscriptionId>,
  pub(crate) output: TxOut,
  pub(crate) previous: Option<InscriptionId>,
  pub(crate) cune: Option<SpacedCune>,
  pub(crate) sat: Option<Sat>,
  pub(crate) satpoint: SatPoint,
  pub(crate) timestamp: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct CraftscriptionJson {
  pub(crate) chain: Chain,
  pub(crate) genesis_fee: u64,
  pub(crate) genesis_height: u32,
  pub(crate) inscription: Inscription,
  pub(crate) inscription_id: InscriptionId,
  pub(crate) inscription_number: u64,
  pub(crate) next: Option<InscriptionId>,
  pub(crate) output: TxOut,
  pub(crate) address: Option<String>,
  pub(crate) previous: Option<InscriptionId>,
  pub(crate) cune: Option<SpacedCune>,
  pub(crate) sat: Option<Sat>,
  pub(crate) satpoint: SatPoint,
  pub(crate) timestamp: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InscriptionJson {
  pub tx_id: String,
  pub vout: u32,
  pub content: Option<Vec<u8>>,
  pub content_length: Option<usize>,
  pub content_type: Option<String>,
  pub genesis_height: u32,
  pub inscription_id: InscriptionId,
  pub inscription_number: u64,
  //pub cune: Option<SpacedCune>,
  pub timestamp: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InscriptionByAddressJson {
  pub utxo: Utxo,
  pub content: Option<String>,
  pub content_length: Option<usize>,
  pub content_type: Option<String>,
  pub genesis_height: u32,
  pub inscription_id: InscriptionId,
  pub inscription_number: u64,
  //pub cune: Option<SpacedCune>,
  pub timestamp: u32,
  pub offset: u64,
}

impl PageContent for InscriptionHtml {
  fn title(&self) -> String {
    format!("Craftscription {}", self.inscription_number)
  }

  fn preview_image_url(&self) -> Option<Trusted<String>> {
    Some(Trusted(format!("/content/{}", self.inscription_id)))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn without_sat_or_nav_links() {
    assert_regex_match!(
      InscriptionHtml {
        genesis_fee: 1,
        inscription: inscription("text/plain;charset=utf-8", "HELLOWORLD"),
        inscription_id: inscription_id(1),
        inscription_number: 1,
        output: tx_out(1, address()),
        satpoint: satpoint(1, 0),
        ..Default::default()
      },
      "
        <h1>Craftscription 1</h1>
        <div class=inscription>
        <div>❮</div>
        <iframe .* src=/preview/1{64}i1></iframe>
        <div>❯</div>
        </div>
        <dl>
          <dt>id</dt>
          <dd class=monospace>1{64}i1</dd>
          <dt>address</dt>
          <dd class=monospace>bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4</dd>
          <dt>output value</dt>
          <dd>1</dd>
          <dt>preview</dt>
          <dd><a href=/preview/1{64}i1>link</a></dd>
          <dt>content</dt>
          <dd><a href=/content/1{64}i1>link</a></dd>
          <dt>content length</dt>
          <dd>10 bytes</dd>
          <dt>content type</dt>
          <dd>text/plain;charset=utf-8</dd>
          <dt>timestamp</dt>
          <dd><time>1970-01-01 00:00:00 UTC</time></dd>
          <dt>genesis height</dt>
          <dd><a href=/block/0>0</a></dd>
          <dt>genesis fee</dt>
          <dd>1</dd>
          <dt>genesis transaction</dt>
          <dd><a class=monospace href=/tx/1{64}>1{64}</a></dd>
          <dt>location</dt>
          <dd class=monospace>1{64}:1:0</dd>
          <dt>output</dt>
          <dd><a class=monospace href=/output/1{64}:1>1{64}:1</a></dd>
          <dt>offset</dt>
          <dd>0</dd>
        </dl>
      "
      .unindent()
    );
  }

  #[test]
  fn with_sat() {
    assert_regex_match!(
      InscriptionHtml {
        genesis_fee: 1,
        inscription: inscription("text/plain;charset=utf-8", "HELLOWORLD"),
        inscription_id: inscription_id(1),
        inscription_number: 1,
        output: tx_out(1, address()),
        sat: Some(Sat(1)),
        satpoint: satpoint(1, 0),
        ..Default::default()
      },
      "
        <h1>Craftscription 1</h1>
        .*
        <dl>
          .*
          <dt>sat</dt>
          <dd><a href=/sat/1>1</a></dd>
          <dt>preview</dt>
          .*
        </dl>
      "
      .unindent()
    );
  }

  #[test]
  fn with_prev_and_next() {
    assert_regex_match!(
      InscriptionHtml {
        genesis_fee: 1,
        inscription: inscription("text/plain;charset=utf-8", "HELLOWORLD"),
        inscription_id: inscription_id(2),
        next: Some(inscription_id(3)),
        inscription_number: 1,
        output: tx_out(1, address()),
        previous: Some(inscription_id(1)),
        satpoint: satpoint(1, 0),
        ..Default::default()
      },
      "
        <h1>Craftscription 1</h1>
        <div class=inscription>
        <a class=prev href=/craftscription/1{64}i1>❮</a>
        <iframe .* src=/preview/2{64}i2></iframe>
        <a class=next href=/craftscription/3{64}i3>❯</a>
        </div>
        .*
      "
      .unindent()
    );
  }

  #[test]
  fn with_cune() {
    assert_regex_match!(
      InscriptionHtml {
        genesis_fee: 1,
        inscription: inscription("text/plain;charset=utf-8", "HELLOWORLD"),
        inscription_id: inscription_id(1),
        inscription_number: 1,
        satpoint: satpoint(1, 0),
        cune: Some(Cune(0)),
        ..Default::default()
      },
      "
        <h1>Craftscription 1</h1>
        .*
        <dl>
          .*
          <dt>cune</dt>
          <dd><a href=/cune/A>A</a></dd>
        </dl>
      "
      .unindent()
    );
  }
}
