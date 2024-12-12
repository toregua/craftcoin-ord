use super::*;

#[derive(Boilerplate, Debug, PartialEq, Serialize, Deserialize)]
pub struct CuneBalancesHtml {
  pub balances: BTreeMap<SpacedCune, BTreeMap<OutPoint, u128>>,
}

impl PageContent for CuneBalancesHtml {
  fn title(&self) -> String {
    "Cune Balances".to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const CUNE: u128 = 99246114928149462;

  #[test]
  fn display_cune_balances() {
    let balances: BTreeMap<Cune, BTreeMap<OutPoint, u128>> = vec![
      (
        Cune(CUNE),
        vec![(
          OutPoint {
            txid: txid(1),
            vout: 1,
          },
          1000,
        )]
        .into_iter()
        .collect(),
      ),
      (
        Cune(CUNE + 1),
        vec![(
          OutPoint {
            txid: txid(2),
            vout: 2,
          },
          12345678,
        )]
        .into_iter()
        .collect(),
      ),
    ]
    .into_iter()
    .collect();

    assert_regex_match!(
      CuneBalancesHtml { balances }.to_string(),
      "<h1>Cune Balances</h1>
<table>
  <tr>
    <th>cune</th>
    <th>balances</th>
  </tr>
  <tr>
    <td><a href=/cune/AAAAAAAAAAAAA>.*</a></td>
    <td>
      <table>
        <tr>
          <td class=monospace>
            <a href=/output/1{64}:1>1{64}:1</a>
          </td>
          <td class=monospace>
            1000
          </td>
        </tr>
      </table>
    </td>
  </tr>
  <tr>
    <td><a href=/cune/AAAAAAAAAAAAB>.*</a></td>
    <td>
      <table>
        <tr>
          <td class=monospace>
            <a href=/output/2{64}:2>2{64}:2</a>
          </td>
          <td class=monospace>
            12345678
          </td>
        </tr>
      </table>
    </td>
  </tr>
</table>
"
      .unindent()
    );
  }
}
