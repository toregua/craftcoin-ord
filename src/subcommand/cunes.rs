use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub cunes: BTreeMap<Cune, CuneInfo>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CuneInfo {
  pub block: u64,
  pub burned: u128,
  pub divisibility: u8,
  pub etching: Txid,
  pub height: u64,
  pub id: CuneId,
  pub index: u32,
  pub terms: Option<Terms>,
  pub mints: u128,
  pub number: u64,
  pub premine: u128,
  pub cune: Cune,
  pub spacers: u32,
  pub supply: u128,
  pub symbol: Option<char>,
  pub timestamp: DateTime<Utc>,
  pub turbo: bool,
  pub tx: u32,
}

pub(crate) fn run(options: Options) -> SubcommandResult {
  let index = Index::open(&options)?;

  ensure!(
    index.has_cune_index(),
    "`ord cunes` requires index created with `--index-cunes` flag",
  );

  index.update()?;

  Ok(Box::new(Output {
    cunes: index
      .cunes()?
      .into_iter()
      .map(
        |(
          id,
          entry @ CuneEntry {
            block,
            burned,
            divisibility,
            etching,
            terms,
            mints,
            number,
            premine,
            cune,
            spacers,
            supply,
            symbol,
            timestamp,
            turbo,
          },
        )| {
          (
            cune,
            CuneInfo {
              block,
              burned,
              divisibility,
              etching,
              height: id.height,
              id,
              index: id.index,
              terms,
              mints,
              number,
              premine,
              timestamp: crate::timestamp(timestamp),
              cune,
              spacers,
              supply,
              symbol,
              turbo,
              tx: id.index,
            },
          )
        },
      )
      .collect::<BTreeMap<Cune, CuneInfo>>(),
  }))
}
