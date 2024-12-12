use crate::sat_point::SatPoint;
use {super::*, crate::wallet::Wallet};

#[derive(Serialize, Deserialize)]
pub struct Output {
  pub inscription: InscriptionId,
  pub location: SatPoint,
  pub explorer: String,
}

pub(crate) fn run(options: Options) -> SubcommandResult {
  let index = Index::open(&options)?;
  index.update()?;

  let inscriptions = index.get_inscriptions(None)?;
  let unspent_outputs = index.get_unspent_outputs(Wallet::load(&options)?)?;

  let explorer = match options.chain() {
    Chain::Mainnet => "https://ordinals.com/craftscription/",
    Chain::Regtest => "http://localhost/craftscription/",
    Chain::Signet => "https://signet.ordinals.com/craftscription/",
    Chain::Testnet => "https://testnet.ordinals.com/craftscription/",
  };

  let mut output = Vec::new();

  for (location, inscription) in inscriptions {
    if unspent_outputs.contains_key(&location.outpoint) {
      output.push(Output {
        location,
        inscription,
        explorer: format!("{explorer}{inscription}"),
      });
    }
  }

  Ok(Box::new(output))
}
