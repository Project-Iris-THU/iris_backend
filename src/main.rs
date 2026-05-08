use crate::argument_parser::parse_arguments;
use crate::config::interface_creator::create_interfaces;
use crate::config::load_config_file::load_config_file;
use crate::config::load_environment::load_environment;
use crate::data::app_state::AppState;
use crate::dns::multicast_advertiser::create_multicast_advertiser;
use crate::web::info::info;
use crate::web::websocket::handler::websocket_handler;
use actix_web::web as actix_web_web;
use actix_web::{App, HttpServer};
use data::defaults::create_default_config_data;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::Arc;

pub mod argument_parser;
pub mod config;
pub mod data;
pub mod dns;
pub mod ml_engines;
pub mod pipeline;
pub mod web;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
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

    let _mdns_service = create_multicast_advertiser(config_data.port)?;

    let interface_config = create_interfaces(config_data)?;

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(actix_web_web::Data::new(AppState {
                interfaces: Arc::new(interface_config.clone()),
            }))
            .service(info)
            .service(websocket_handler)
    });

    let host = &config_data.host;
    let port = config_data.port;

    if config_data.tls.enabled {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
        builder.set_private_key_file(&config_data.tls.key_path, SslFiletype::PEM)?;
        builder.set_certificate_chain_file(&config_data.tls.cert_path)?;
        server = server.bind_openssl(format!("{host}:{port}"), builder)?
    } else {
        server = server.bind((config_data.host.clone(), config_data.port))?;
    }

    server.run().await?;

    Ok(())
}
