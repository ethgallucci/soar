// TODO: Allow user to input RPC endpoints instead of pigeon-holing them to one
// Finish writing tests for currently supported chains
// Add many more chains...
mod registry;

fn main() {
    pretty_env_logger::init();
    log::info!("soar to new heights...");

    registry_serialization()
}

fn registry_serialization() {
    use registry::*;

    let registry = Registry::new();
    let mut chain = Chain::new(registry.recent[0].to_owned());
    let rpc_e = chain.parse_rpc();

    log::info!("Serialized Registry (snippet): {:?}", chain.meta);
}

#[cfg(test)]
mod tests {
    use super::registry::*;

    #[test]
    fn registry_serialization() {
        let registry = Registry::new();
        let chain = Chain::new(registry.recent[6].to_owned());

        log::info!("Serialized Registry (snippet): {:?}", chain.meta);
    }
}
