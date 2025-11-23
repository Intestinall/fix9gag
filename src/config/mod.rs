mod config_struct;
mod validators;

pub use crate::config::config_struct::Config;
use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use std::process::exit;
use validator::Validate;

pub fn get_config() -> Config {
    let config = match Figment::from(Serialized::defaults(Config::default()))
        .merge(Toml::file("fix9gag.toml"))
        .merge(Env::prefixed("FIX9GAG_"))
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
