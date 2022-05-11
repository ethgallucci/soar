// TODO: Allow user to input RPC endpoints instead of pigeon-holing them to one
// Finish writing tests for currently supported chains
// Add many more chains...

use std::env;

mod registry;
use registry::{ChainRPC, Chain};
mod flags;
use flags::{parse_chain, parse_flags, from_flag, usage};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.iter().len() > 2 {
        let chain = parse_chain(args.clone()).unwrap();
        let query = from_flag(parse_flags(args.clone()).unwrap()).unwrap();

        let rpc = ChainRPC::new_and_launch(chain, &query.as_str()).unwrap();
        let res = rpc.last_response.unwrap();
        println!("{}", res)
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
        let akt = ChainRPC::new_and_launch(Chain::Akash, "block_results?height=1090").unwrap();
        println!("{}", akt.last_response.unwrap())
    }

    #[test]
    fn evmos() {
        let evmos = ChainRPC::new_and_launch(Chain::Evmos, "abci_info?").unwrap();
        println!("{}", evmos.last_response.unwrap())
    }

    #[test]
    fn gaia() {
        let gaia = ChainRPC::new_and_launch(Chain::Gaia, "unconfirmed_txs?limit=100").unwrap();
        println!("{}", gaia.last_response.unwrap())
    }

    #[test]
    fn juno() {
        let juno = ChainRPC::new_and_launch(Chain::Juno, "block_results?height=2578099").unwrap(); 
        println!("{}", juno.last_response.unwrap())
    }

    #[test]
    fn regen() {
        let regen = ChainRPC::new_and_launch(Chain::Regen, "status?").unwrap();
        println!("{}", regen.last_response.unwrap())
    }
}
