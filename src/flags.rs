#![allow(dead_code)]
use super::Chain;

#[derive(Debug)]
pub struct Default {}

#[derive(Debug)]
pub struct ParseError {}

pub fn usage() {
    println!("soar is an rpc command line tool for the interchain\n\n\
    usage:\nsoar [chain] --block-by-height [height]
    ");
    println!("flags:\n\n--abci-info\n--block-by-height [height]");
    println!("--block-by-hash [hash]\n--block-results [height]\n--block-search [query] [page] [per_page] [order_by]\n\
    --blockchain [min_height] [max_height]\n--broadcast-evidence [evidence]\n--broadcast-tx-async [tx]\n\
    --unconfirmed-txs [limit]
    ");
}

pub fn parse_chain(args: Vec<String>) -> Option<Chain> {
    if args.iter().len() == 1 {
        usage();
        None
    } else {
        match &args[1] as &str {
            "akash" => Some(Chain::Akash),
            "gaia" => Some(Chain::Gaia),
            "juno" => Some(Chain::Juno),
            "regen" => Some(Chain::Regen),
            "osmo" => Some(Chain::Osmosis),
            "osmosis" => Some(Chain::Osmosis),
            "secret" => Some(Chain::Secret),
            "stargaze" => Some(Chain::Stargaze),
            "terra" => Some(Chain::Terra),
            _ => None,
        }
    }
}

pub enum QueryFlag {
    AbciInfo,
    AbciQuery { path: String, data: String, height: String, prove: String},
    BlockByHeight { height: String },
    BlockByHash { hash: String },
    BlockResults { height: String },
    BlockSearch { query: String, page: String, per_page: String, order_by: String },
    BlockchainByHeight { min_height: String, max_height: String },
    BroadcastEvidence { evidence: String },
    BroadcastTxAsync { tx: String },
    BroadcastTxCommit { tx: String },
    BroadcastTxSync { tx: String },
    CheckTx { tx: String },
    CommitByHeight { height: String },
    ConsensusParamsByHeight { height: String },
    ConsensusState,
    UnconfirmedTxs { limit: String },
}

pub fn from_flag(flag: QueryFlag) -> Option<String> {
    match flag {
        QueryFlag::AbciInfo => Some("abci_info?".to_string()),
        QueryFlag::AbciQuery{ path, data, height, prove} => {
            Some(format!("abci_query?path={}&data={}&height={}&prove={}", path, data, height, prove))
        },
        QueryFlag::BlockByHeight { height } => Some(format!("block?height={}", height)),
        QueryFlag::BlockByHash { hash } => Some(format!("block_by_hash?hash={}", hash)),
        QueryFlag::BlockResults { height } => Some(format!("block_results?height={}", height)),
        QueryFlag::BlockSearch { query, page, per_page, order_by } => {
            Some(format!("block_search?query={}&page={}&per_page={}&order_by={}", query, page, per_page, order_by))
        },
        QueryFlag::BlockchainByHeight{ min_height, max_height } => {
            Some(format!("blockchain?minHeight={}&maxHeight={}", min_height, max_height))
        },
        QueryFlag::BroadcastEvidence { evidence } => Some(format!("broadcast_evidence?evidence={}", evidence)),
        QueryFlag::BroadcastTxAsync { tx } => Some(format!("broadcast_tx_async?tx={}", tx)),
        QueryFlag::BroadcastTxCommit { tx } => Some(format!("broadcast_tx_commit?tx={}", tx)),
        QueryFlag::BroadcastTxSync { tx } => Some(format!("broadcast_tx_sync?tx={}", tx)),
        QueryFlag::CheckTx { tx } => Some(format!("check_tx?tx={}", tx)),
        QueryFlag::CommitByHeight { height } => Some(format!("commit?height={}", height)),
        QueryFlag::ConsensusParamsByHeight { height } => Some(format!("consensus_params?height={}", height)),
        QueryFlag::ConsensusState => None,
        QueryFlag::UnconfirmedTxs { limit } => Some(format!("unconfirmed_txs?limit={}", limit)),
    }
}

pub fn parse_flags(args: Vec<String>) -> Result<QueryFlag, ParseError> {
    if args.iter().len() > 1 {
        match &args[2] as &str {
            "--abci-info" => Ok(QueryFlag::AbciInfo),
            "--block-by-height" => Ok(QueryFlag::BlockByHeight { height: args[3].clone() }),
            "--block-by-hash" => Ok(QueryFlag::BlockByHash { hash: args[3].clone() }),
            "--block-results" => Ok(QueryFlag::BlockResults { height: args[3].clone() }),
            "--block--search" => Ok(QueryFlag::BlockSearch { 
                query: args[3].clone() , 
                page: args[4].clone(), 
                per_page: args[5].clone(),
                order_by: args[6].clone(),
            }),
            "--blockchain" => Ok(QueryFlag::BlockchainByHeight { 
                min_height: args[3].clone(),
                max_height: args[4].clone(),
            }),
            "--broadcast-evidence" => Ok(QueryFlag::BroadcastEvidence { evidence: args[3].clone() }),
            "--broadcast-tx-async" => Ok(QueryFlag::BroadcastTxAsync { tx: args[3].clone() }),
            "--unconfirmed-txs" => Ok(QueryFlag::UnconfirmedTxs { limit: args[3].clone() }),

            _ => Err(ParseError {}),
        }
    } else {
        Err(ParseError {})
    }
}
