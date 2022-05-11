use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use ureq;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GenericErr {
    PrimeErr,
    QueryErr,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Chain {
    Akash,
    Evmos,
    Gaia,
    Juno,
    Osmosis,
    Regen,
    Secret,
    Sentinel,
    Stargaze,
    Terra,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChainRPC {
    pub endpoint: String,
    pub last_response: Option<String>,
}

impl ChainRPC {
    pub fn new_and_launch(
        chain: Chain,
        query: &str,
    ) -> Result<ChainRPC, Box<dyn std::error::Error>> {
        match chain {
            Chain::Akash => Ok(ChainRPC::akash(query)),
            Chain::Evmos => Ok(ChainRPC::evmos(query)),
            Chain::Gaia => Ok(ChainRPC::gaia(query)),
            Chain::Juno => Ok(ChainRPC::juno(query)),
            Chain::Regen => Ok(ChainRPC::regen(query)),
            Chain::Osmosis => Ok(ChainRPC::osmosis(query)),
            Chain::Secret => Ok(ChainRPC::secret(query)),
            Chain::Sentinel => unimplemented!() ,
            Chain::Stargaze => Ok(ChainRPC::stargaze(query)),
            Chain::Terra => Ok(ChainRPC::terra(query)),
        }
    }

    pub fn launch_from_endpoint(endpoint: &str, q: &str) -> ChainRPC {
        let pretty = format_response(endpoint.clone(), q).unwrap();
        ChainRPC {
            endpoint: endpoint.to_string(),
            last_response: Some(pretty),
        }
    }

    // Chains //////////////////////////////////////////////////////////////////
    // TODO: Impelement the end point functions as a user choice
    // e.g. user's should input a node they want to connect to
    fn akash(q: &str) -> ChainRPC {
        let endpoint = "https://rpc.akash.forbole.com/";
        ChainRPC::launch_from_endpoint(endpoint, q)
    }

    fn evmos(q: &str) -> ChainRPC {
        let endpoint = "https://rpc-osmosis.blockapsis.com/";
        ChainRPC::launch_from_endpoint(endpoint, q)
    }

    fn gaia(q: &str) -> ChainRPC {
        let endpoint = "https://rpc.cosmos.network:443/";
        ChainRPC::launch_from_endpoint(endpoint, q)
    }

    fn juno(q: &str) -> ChainRPC {
        let endpoint = "https://rpc.juno.pupmos.network/";
        ChainRPC::launch_from_endpoint(endpoint, q)
    }

    fn osmosis(q: &str) -> ChainRPC {
        let endpoint = "https://rpc-osmosis.blockapsis.com/";
        ChainRPC::launch_from_endpoint(endpoint, q)
    }

    fn regen(q: &str) -> ChainRPC {
        let endpoint = "http://public-rpc.regen.vitwit.com:26657/";
        ChainRPC::launch_from_endpoint(endpoint, q)
    }

    fn secret(q: &str) -> ChainRPC {
        let endpoint = "https://rpc-secret.scrtlabs.com/secret-4/rpc/";
        ChainRPC::launch_from_endpoint(endpoint, q)
    }

    fn stargaze(q: &str) -> ChainRPC {
        let endpoint = "https://rpc.stargaze-apis.com/";
        ChainRPC::launch_from_endpoint(endpoint, q)
    }

    fn terra(q: &str) -> ChainRPC {
        let endpoint = "https://terra-rpc.easy2stake.com/";
        ChainRPC::launch_from_endpoint(endpoint, q)
    }
}

// launch a query with a formatted response ////////////////////////////////////////////////////////////////
pub fn format_response(e: &str, q: &str) -> Result<String, Box<dyn std::error::Error>> {
    let full = format!("{}{}", e, q);
    let res = ureq::get(&full).call()?.into_string()?;
    let j: serde_json::Value = serde_json::from_str(&res)?;
    let pretty = to_string_pretty(&j)?;
    Ok(pretty)
}
