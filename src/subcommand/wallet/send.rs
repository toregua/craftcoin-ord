use bitcoin::PackedLockTime;
use {super::*, crate::wallet::Wallet};

#[derive(Debug, Parser)]
pub(crate) struct Send {
  address: Address,
  outgoing: Outgoing,
  #[arg(long, help = "Use fee rate of <FEE_RATE> sats/vB")]
  fee_rate: FeeRate,
}

#[derive(Serialize, Deserialize)]
pub struct Output {
  pub transaction: Txid,
}

impl Send {
  pub(crate) fn run(self, options: Options) -> SubcommandResult {
    let address = self.address.clone();

    let index = Index::open(&options)?;
    index.update()?;

    let client = options.craftcoin_rpc_client_for_wallet_command(false)?;

    let unspent_outputs = index.get_unspent_outputs(Wallet::load(&options)?)?;

    let inscriptions = index.get_inscriptions(None)?;

    let dunic_outputs =
      index.get_dunic_outputs(&unspent_outputs.keys().cloned().collect::<Vec<OutPoint>>())?;

    let satpoint = match self.outgoing {
      Outgoing::Amount(amount) => {
        let transaction = Self::send_amount(&client, amount, address, self.fee_rate)?;
        return Ok(Box::new(Output { transaction }));
      }
      Outgoing::InscriptionId(id) => index
        .get_inscription_satpoint_by_id(id)?
        .ok_or_else(|| anyhow!("inscription {id} not found"))?,
      Outgoing::Cune { decimal, cune } => {
        let transaction = Self::send_cunes(
          address,
          &client,
          decimal,
          self.fee_rate,
          &index,
          inscriptions,
          cune,
          dunic_outputs,
          unspent_outputs,
        )?;
        return Ok(Box::new(Output { transaction }));
      }
      Outgoing::SatPoint(satpoint) => {
        for inscription_satpoint in inscriptions.keys() {
          if satpoint == *inscription_satpoint {
            bail!("inscriptions must be sent by inscription ID");
          }
        }

        ensure!(
          !dunic_outputs.contains(&satpoint.outpoint),
          "dunic outpoints may not be sent by satpoint"
        );

        satpoint
      }
    };

    let change = [get_change_address(&client)?, get_change_address(&client)?];

    let unsigned_transaction = TransactionBuilder::build_transaction_with_postage(
      satpoint,
      inscriptions,
      unspent_outputs,
      dunic_outputs,
      self.address,
      change,
      self.fee_rate,
    )?;

    let signed_tx = client
      .sign_raw_transaction_with_wallet(&unsigned_transaction, None, None)?
      .hex;

    let txid = client.send_raw_transaction(&signed_tx)?;

    println!("{txid}");

    Ok(Box::new(Output { transaction: txid }))
  }

  fn send_amount(
    client: &Client,
    amount: Amount,
    address: Address,
    fee_rate: FeeRate,
  ) -> Result<Txid> {
    Ok(client.call(
      "sendtoaddress",
      &[
        address.to_string().into(), //  1. address
        amount.to_btc().into(),     //  2. amount
        serde_json::Value::Null,    //  3. comment
        serde_json::Value::Null,    //  4. comment_to
        serde_json::Value::Null,    //  5. subtractfeefromamount
        serde_json::Value::Null,    //  6. replaceable
        serde_json::Value::Null,    //  7. conf_target
        serde_json::Value::Null,    //  8. estimate_mode
        serde_json::Value::Null,    //  9. avoid_reuse
        fee_rate.n().into(),        // 10. fee_rate
      ],
    )?)
  }

  fn send_cunes(
    address: Address,
    client: &Client,
    decimal: Decimal,
    fee_rate: FeeRate,
    index: &Index,
    inscriptions: BTreeMap<SatPoint, InscriptionId>,
    spaced_cune: SpacedCune,
    dunic_outputs: BTreeSet<OutPoint>,
    unspent_outputs: BTreeMap<OutPoint, Amount>,
  ) -> Result<Txid> {
    ensure!(
      index.has_cune_index(),
      "sending cunes with `ord send` requires index created with `--index-cunes` flag",
    );

    let (id, entry) = index
      .cune(spaced_cune.cune)?
      .with_context(|| format!("cune `{}` has not been etched", spaced_cune.cune))?;

    let amount = decimal.to_amount(entry.divisibility)?;

    let inscribed_outputs = inscriptions
      .keys()
      .map(|satpoint| satpoint.outpoint)
      .collect::<HashSet<OutPoint>>();

    let mut input_cunes = 0;
    let mut input = Vec::new();

    for output in dunic_outputs {
      if inscribed_outputs.contains(&output) {
        continue;
      }

      let balance = index.get_cune_balance(output, id)?;

      if balance > 0 {
        input_cunes += balance;
        input.push(output);
      }

      if input_cunes >= amount {
        break;
      }
    }

    ensure! {
      input_cunes >= amount,
      "insufficient `{}` balance, only {} in wallet",
      spaced_cune,
      Pile {
        amount: input_cunes,
        divisibility: entry.divisibility,
        symbol: entry.symbol
      },
    }

    let cunestone = Cunestone {
      edicts: vec![Edict {
        amount,
        id: id.into(),
        output: 2,
      }],
      ..Default::default()
    };

    let unfunded_transaction = Transaction {
      version: 1,
      lock_time: PackedLockTime::ZERO,
      input: input
        .into_iter()
        .map(|previous_output| TxIn {
          previous_output,
          script_sig: Script::new(),
          sequence: Sequence::MAX,
          witness: Witness::new(),
        })
        .collect(),
      output: vec![
        TxOut {
          script_pubkey: cunestone.encipher(),
          value: 0,
        },
        TxOut {
          script_pubkey: get_change_address(client)?.script_pubkey(),
          value: TARGET_POSTAGE.to_sat(),
        },
        TxOut {
          script_pubkey: address.script_pubkey(),
          value: TARGET_POSTAGE.to_sat(),
        },
      ],
    };

    let unsigned_transaction = fund_raw_transaction(client, fee_rate, &unfunded_transaction)?;

    let signed_transaction = client
      .sign_raw_transaction_with_wallet(&unsigned_transaction, None, None)?
      .hex;

    Ok(client.send_raw_transaction(&signed_transaction)?)
  }
}
