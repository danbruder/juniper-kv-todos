use kv::*;

pub fn config() -> Config {
    Config {
        path: "db.sled".into(),
        read_only: false,
        temporary: false,
        use_compression: false,
        flush_every_ms: None,
    }
}
