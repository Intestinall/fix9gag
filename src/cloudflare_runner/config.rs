use crate::Config;
use figment::{Figment, providers::Serialized};
use std::process::exit;
use validator::Validate;

use figment::value::Value;
use figment::{
    Error, Metadata, Profile, Provider,
    value::{Dict, Map},
};

pub struct WorkerEnvProvider<'a> {
    env: &'a worker::Env,
}

impl<'a> Provider for WorkerEnvProvider<'a> {
    fn metadata(&self) -> Metadata {
        Metadata::named("WorkerEnvProvider")
    }
    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        let mut sub_map: Dict = Map::new();
        let keys = [
            "host",
            "port",
            "hostname",
            "oembed_endpoint",
            "provider_name",
            "provider_url",
            "embed_color",
        ];
        for key in keys {
            if let Ok(val) = self.env.var(&key.to_ascii_uppercase()) {
                sub_map.insert(
                    key.to_ascii_lowercase(),
                    Value::from(val.to_string()).deserialize().unwrap(),
                );
            };
        }
        let mut map: Map<Profile, Dict> = Map::new();
        map.insert(Profile::default(), sub_map);
        Ok(map)
    }
}

pub fn get_config(env: worker::Env) -> Config {
    let config = match Figment::from(Serialized::defaults(Config::default()))
        .merge(WorkerEnvProvider { env: &env })
        .extract::<Config>()
    {
        Ok(config) => config,
        Err(err) => {
            println!("{}: {}", err.path[0], err.kind);
            exit(1);
        }
    };

    match config.validate() {
        Ok(()) => config,
        Err(err) => {
            for (field, errors) in err.field_errors() {
                for error in errors {
                    println!("{}: {}", field, &error.code);
                }
            }
            exit(1)
        }
    }
}
