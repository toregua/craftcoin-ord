use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub cunes: BTreeMap<SpacedCune, BTreeMap<OutPoint, u128>>,
}

pub(crate) fn run(options: Options) -> SubcommandResult {
  let index = Index::open(&options)?;

  ensure!(
    index.has_cune_index(),
    "`ord balances` requires index created with `--index-cunes` flag",
  );

  index.update()?;

  Ok(Box::new(Output {
    cunes: index.get_cune_balance_map()?,
  }))
}
