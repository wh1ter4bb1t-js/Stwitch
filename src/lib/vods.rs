use twitch_api2::helix::videos::get_videos;

pub struct Vods {
    pub vods: Vec<get_videos::Video>
}

impl Vods {
    pub fn get_titles(&self) -> Vec<&String> {
        let mut titles = vec![];
        for vod in &self.vods {
            titles.push(&vod.title);
        }
        return titles;
    }

    pub fn get_stream(&self, title: String) -> String {
        let mut url = "".to_string();
        for vod in &self.vods {
            if title == vod.title {
                url = vod.url.clone();
            }
        }

        return url;
    }
}
