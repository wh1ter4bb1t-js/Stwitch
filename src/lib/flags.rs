use crate::Config;
use dialoguer::Select;

pub fn handle_flags(cfg: &mut Config, search_items: Vec<String>) -> (bool, String) {
    let mut flags = vec![];
    let mut q = vec![];

    for arg in &search_items[1..] {
        match arg.as_ref() {
            "-d"| "--detach" => {
                flags.push("-d");
            },
            "-s" | "--subscribe" => {
                flags.push("-s")
            },
            "-sd" | "-ds" => {
                flags.push("-d");
                flags.push("-s")
            },
            _ => {
                q.push(arg.to_string());
            }
        }
    }

    let detach = if flags.contains(&"-d") {
        true
    } else {
        false
    } ;

    let caster_name = if flags.contains(&"-s") {
        let caster_choice = if q.len() > 0 {
            cfg.subscribes.push(q.join(" "));
            confy::store("stwitch", &cfg).expect("failed to store a new config");
            q.join(" ")
        } else {
            let streamer_choices = &cfg.subscribes;
            let streamer_selection = Select::new()
                .items(&streamer_choices)
                .default(0)
                .interact()
                .expect("failed to get streamer_selection");

            streamer_choices[streamer_selection].to_string()
        };

        caster_choice
    } else {
        q.join(" ")
    };

    return (detach, caster_name)
}
