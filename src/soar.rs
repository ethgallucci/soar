// TODO: Continue prototyping v0.2.0
//  Build out composable rpc query methods for the Chain struct
////////////////////////////////////////////////////////////////
/// FIXME: Registry dirwalker has a breaking bug, it's only
///   parsing a single folder's chain.json. Not very dirwalker like!
///   Need to patch that and further iterate on the Chain.parse_rpc()
///   method to isolate the url address from the json array before
///   building out the composable rpc query methods
mod registry;

fn main() {
    pretty_env_logger::init();
    log::info!("soar to new heights...");

    try_registry_sync()
}

fn try_registry_sync() {
    use registry::*;

    let registry = Registry::new();
    let mut chain = Chain::new(registry.recent[0].to_owned());
    let _rpc_e = chain.parse_rpc();

    log::info!("Serialized Registry (snippet): {:?}", chain.meta);
}

#[cfg(test)]
mod tests {
    use super::registry::*;

    #[test]
    fn registry_sync_test() {
        let registry = Registry::new();
        let chain = Chain::new(registry.recent[6].to_owned());

        log::info!("Serialized Registry (snippet): {:?}", chain.meta);
    }
}
