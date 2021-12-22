use crate::lib::vods::Vods;
use dialoguer::Select;
use twitch_api2::helix::streams::get_streams;

pub fn handle_selection(caster_name: String, stream_details: Vec<get_streams::Stream>, caster_vods: Vods) -> String {
    let mut vods_titles = caster_vods.get_titles();
    let s = if stream_details.len() > 0 { format!("LIVE: {}", stream_details[0].title) } else { "".to_string() }.to_string();
    if s.len() > 0 {
        vods_titles.insert(0, &s)
    };

    let selection = Select::new()
        .items(&vods_titles)
        .default(0)
        .interact()
        .expect("failed to get title selection");

    let url = if selection == 0 && vods_titles[selection].contains("LIVE") {
        format!("https://twitch.tv/{}", caster_name)
    } else {
        if s.len() > 0 {
            caster_vods.get_stream(vods_titles[selection+1].to_string())
        } else {
            caster_vods.get_stream(vods_titles[selection].to_string())
        }
    }.to_string();

    return url;
}
