#[derive(Debug, Clone)]
pub enum TendermintQuery {
    Syncing,
    BlocksLatest,
    BlocksHeight { height: String },
    ValidatorsLatest,
    ValidatorsHeight { height: String },
}

pub fn tendermint_query_parse(args: Vec<String>) -> Result<TendermintQuery, ()> {
    match &args[0] as &str {
        "--syncing" => Ok(TendermintQuery::Syncing),
        "--blocks-latest" => Ok(TendermintQuery::BlocksLatest),
        "--blocks-height" => Ok(TendermintQuery::BlocksHeight {
            height: args[1].to_owned(),
        }),
        "--validators-latest" => Ok(TendermintQuery::ValidatorsLatest),
        "--validators-height" => Ok(TendermintQuery::ValidatorsHeight {
            height: args[1].to_owned(),
        }),
        _ => Err(()),
    }
}
