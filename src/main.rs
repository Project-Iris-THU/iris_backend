use crate::argument_parser::argument_parser::parse_arguments;
use crate::config::data::create_default_config_data;
use crate::config::interface_creator::create_interfaces;
use crate::config::load_config_file::load_config_file;
use crate::config::load_environment::load_environment;
use crate::dns::multicast_advertiser::create_multicast_advertiser;

pub mod argument_parser;
pub mod config;
pub mod dns;
pub mod ml_engines;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_arguments();
    let config_data = &mut create_default_config_data();

    let mdns_service = create_multicast_advertiser()?;

    if !args.config_file.is_empty() {
        let config_file = match std::fs::File::open(args.config_file) {
            Ok(content) => content,
            Err(e) => Err(e)?,
        };

        load_config_file(config_file, config_data)?;
    }

    load_environment(config_data)?;

    let interface_config = create_interfaces(config_data)?;

    Ok(())
}
