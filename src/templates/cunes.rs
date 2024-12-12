use super::*;

#[derive(Boilerplate)]
pub(crate) struct CunesHtml {
  pub(crate) entries: Vec<(CuneId, CuneEntry)>,
}

impl PageContent for CunesHtml {
  fn title(&self) -> String {
    "Cunes".to_string()
  }
}
