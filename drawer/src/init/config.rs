use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use user_config::Config;

fn get_config_path() -> PathBuf {
    let root_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(root_dir)
        .join("target")
        .join("sample-config.json")
}

pub fn default_config_source() -> Result<BufReader<File>, String> {
    let config_path = get_config_path();
    let file = File::open(config_path).map_err(|e| format!("While opening config file: {e:?}"))?;
    Ok(BufReader::new(file))
}

pub fn get_config<F, R>(config_source: F) -> Result<Config, String>
where
    R: BufRead,
    F: FnOnce() -> Result<R, String>,
{
    let reader = config_source().map_err(|e| format!("While getting config source: {e:?}"))?;
    let config = serde_json::from_reader(reader)
        .map_err(|e| format!("While parsing json from reader: {e:?}"))?;

    Ok(config)
}
