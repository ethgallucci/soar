#[cfg(test)]
mod tests {
    use soar_rpc::registry::{Chain, Registry};

    #[test]
    fn registry_sync_test() {
        std::env::set_var("RUST_LOG", "info");
        pretty_env_logger::init();

        let registry = Registry::new();
        let chain = Chain::new(registry.recent[6].to_owned());

        log::debug!("Serialized Registry (snippet): {:?}", chain.meta);
    }
}
