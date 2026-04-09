use crate::argument_parser::argument_parser::parse_arguments;
use crate::config::data::create_default_config_data;
use crate::config::load_config_file::load_config_file;
use crate::config::load_environment::load_environment;

pub mod argument_parser;
pub mod config;
pub mod dns;
pub mod ml_engines;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_arguments();
    let config_data = &mut create_default_config_data();

    if !args.config_file.is_empty() {
        let config_file = match std::fs::File::open(args.config_file) {
            Ok(content) => content,
            Err(e) => Err(e)?,
        };

        load_config_file(config_file, config_data)?;
    }

    load_environment(config_data)?;

    Ok(())
}
