use std::env;
use serde::{Serialize, Deserialize};
use std::process::Command;
use dialoguer::Select;
use reqwest::Client as ReqwestClient;
use twitch_api2::{TwitchClient, helix::channels::GetChannelInformationRequest, helix::streams::get_streams, helix::videos::get_videos};
use twitch_api2::twitch_oauth2::{AppAccessToken, AccessToken, UserToken, ClientSecret, ClientId, Scope, tokens::errors::TokenError};
use twitch_api2::twitch_oauth2::client::surf_http_client;

struct Vods {
    vods: Vec<get_videos::Video>
}

impl Vods {
    fn get_titles(&self) -> Vec<&String> {
        let mut titles = vec![];
        for vod in &self.vods {
            titles.push(&vod.title);
        }
        return titles;
    }

    fn get_stream(&self, title: String) -> String {
        let mut url = "".to_string();
        for vod in &self.vods {
            if title == vod.title {
                url = vod.url.clone();
            }
        }

        return url;
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    client_id: String,
    secret: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self { Self { client_id: "".into(), secret: "".into() }}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let cfg: Config = confy::load("stwitch")?;
    let search_items: Vec<String> = env::args().collect();

    if search_items.len() < 2 {
        return Ok(())
    };

    let caster_name = search_items[1..].to_vec().join(" ");

    let client_id = cfg.client_id;
    let secret = cfg.secret;

    let token =
    match AppAccessToken::get_app_access_token(surf_http_client, ClientId::new(client_id), ClientSecret::new(secret), Scope::all()).await {
        Ok(t) => t,
        Err(TokenError::Request(e)) => panic!("got error: {:?}", e),
        Err(e) => panic!("{}", e),
    };

    let client: TwitchClient<'static, reqwest::Client> = TwitchClient::new();

    let caster = client.helix.get_channel_from_login(&*caster_name, &token).await?.expect("failed to get user");
    let caster_id = &caster.broadcaster_id;
    
    let stream_request = get_streams::GetStreamsRequest::builder()
        .user_login(vec![caster_name.to_string()])
        .build();

    let stream_details: Vec<get_streams::Stream> = client.helix.req_get(stream_request, &token).await?.data;


    let mut caster_vods = Vods {
        vods: vec![],
    };

    let request = get_videos::GetVideosRequest::builder()
        .user_id(caster_id.to_string())
        .build();
    caster_vods.vods = client.helix.req_get(request, &token).await?.data;

    let mut vods_titles = caster_vods.get_titles();
    let s = if stream_details.len() > 0 { format!("LIVE: {}", stream_details[0].title) } else { "".to_string() }.to_string();
    if s.len() > 0 {
        vods_titles.insert(0, &s)
    };

    let selection = Select::new()
        .items(&vods_titles)
        .default(0)
        .interact()?;

    let url = if selection == 0 && vods_titles[selection].contains("LIVE") {
        format!("https://twitch.tv/{}", caster_name)
    } else {
        if s.len() > 0 {
            caster_vods.get_stream(vods_titles[selection+1].to_string())
        } else {
            caster_vods.get_stream(vods_titles[selection].to_string())
        }
    }.to_string();


    Command::new("mpv").arg(url).status().expect("failed to open url in mpv");

    Ok(())
}
