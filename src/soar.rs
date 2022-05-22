// TODO: Allow user to input RPC endpoints instead of pigeon-holing them to one
// Finish writing tests for currently supported chains
// Add many more chains...

use std::env;

mod registry;
use registry::{ChainRPC, Chain};
mod flags;
use flags::{parse_chain, parse_flags, from_flag, usage};

fn main() {
    pretty_env_logger::init();
    let args: Vec<String> = env::args().collect();
    
    if args.iter().len() > 2 {
        if let Some(chain) = parse_chain(args.clone()) {
            let chain_choice = chain;
            let query = from_flag(parse_flags(args).unwrap()).unwrap();
            let rpc = ChainRPC::new_and_launch_from_chain(chain_choice, &query.as_str()).unwrap();
            let res = rpc.last_response.unwrap();
            log::info!("Response received: {}", res)
        } else {
            let provided_endpoint = args[1].clone();
            let query = from_flag(parse_flags(args).unwrap()).unwrap();
            let rpc = ChainRPC::from(provided_endpoint.clone());
            let res = rpc.launch(&query).unwrap();
            log::info!("Response received: {}", res);
            log::info!("From endpoint: {}", provided_endpoint)
        }
    } else {
        usage()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use registry::{ChainRPC, Chain};
    
    #[test]
    fn akash() {
        let akt = ChainRPC::new_and_launch_from_chain(Chain::Akash, "block_results?height=1090").unwrap();
        println!("{}", akt.last_response.unwrap())
    }

    #[test]
    fn evmos() {
        let evmos = ChainRPC::new_and_launch_from_chain(Chain::Evmos, "abci_info?").unwrap();
        println!("{}", evmos.last_response.unwrap())
    }

    #[test]
    fn gaia() {
        let gaia = ChainRPC::new_and_launch_from_chain(Chain::Gaia, "unconfirmed_txs?limit=100").unwrap();
        println!("{}", gaia.last_response.unwrap())
    }

    #[test]
    fn juno() {
        let juno = ChainRPC::new_and_launch_from_chain(Chain::Juno, "block_results?height=2578099").unwrap(); 
        println!("{}", juno.last_response.unwrap())
    }

    #[test]
    fn regen() {
        let regen = ChainRPC::new_and_launch_from_chain(Chain::Regen, "status?").unwrap();
        println!("{}", regen.last_response.unwrap())
    }
}
