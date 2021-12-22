use crate::lib::vods::Vods;
use twitch_api2::{TwitchClient, helix::streams::get_streams, helix::videos::get_videos};
use twitch_api2::twitch_oauth2::AppAccessToken;


#[warn(private_in_public)]
pub async fn handle_twitch_api(token: AppAccessToken, caster_name: &String) -> Result<(Vec<get_streams::Stream>, Vods), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut caster_vods = Vods {
        vods: vec![],
    };

    let client: TwitchClient<'static, reqwest::Client> = TwitchClient::new();

    let caster = client.helix.get_channel_from_login(&*caster_name, &token)
        .await?
        .expect("failed to get user");

    let caster_id = &caster.broadcaster_id;
    
    let stream_request = get_streams::GetStreamsRequest::builder()
        .user_login(vec![caster_name.to_string()])
        .build();

    let stream_details: Vec<get_streams::Stream> = client.helix.req_get(stream_request, &token)
        .await?
        .data;


    let request = get_videos::GetVideosRequest::builder()
        .user_id(caster_id.to_string())
        .build();

    caster_vods.vods = client.helix.req_get(request, &token)
        .await?
        .data;

    return Ok((stream_details, caster_vods))
}
