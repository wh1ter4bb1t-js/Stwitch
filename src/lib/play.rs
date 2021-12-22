use std::process::Command;

pub fn play(detach: bool, url: String) {
    if detach == true {
        Command::new("mpv")
            .arg("--no-terminal")
            .arg(url)
            .spawn()
            .expect("failed to open url in mpv");
    } else {
        Command::new("mpv")
            .arg(url)
            .status()
            .expect("failed to open url in mpv");
    }
}
