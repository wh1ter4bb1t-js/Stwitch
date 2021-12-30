mod lib;
use std::env;
use lib::twitch::handle_twitch_api;
use lib::flags::{show_help, handle_flags};
use lib::selection::handle_selection;
use lib::play::play;
use lib::config::Config;
use twitch_api2::twitch_oauth2::{AppAccessToken, ClientSecret, ClientId, Scope, tokens::errors::TokenError};
use twitch_api2::twitch_oauth2::client::surf_http_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut cfg: Config = confy::load("stwitch")?;
    let search_items: Vec<String> = env::args().collect();

    if search_items.len() < 2 {
        println!("Please make sure to add a query\n");
        show_help();
        return Ok(())
    };

    let (detach, caster_name) = handle_flags(&mut cfg, search_items);
    let client_id = cfg.client_id;
    let secret = cfg.secret;


    let token =
    match AppAccessToken::get_app_access_token(
            surf_http_client, 
            ClientId::new(client_id.to_string()), 
            ClientSecret::new(secret.to_string()), 
            Scope::all()
        ).await {
            Ok(t) => t,
            Err(TokenError::Request(e)) => panic!("got error: {:?}", e),
            Err(e) => panic!("{}", e),
    };



    let (stream_details, caster_vods) = handle_twitch_api(token, &caster_name)
        .await
        .expect("failed to get stream details");

    let url = handle_selection(caster_name, stream_details, caster_vods);

    play(detach, url);

    Ok(())
}
