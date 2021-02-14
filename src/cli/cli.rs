use clap::{clap_app, crate_authors, crate_version, App};

pub fn build_cli() -> App<'static, 'static> {
    clap_app!(npc =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Nispor CLI")
        (@arg ifname: [INTERFACE_NAME] "interface name")
        (@arg json: -j --json "Show in json format")
        (@subcommand route =>
            (@arg json: -j --json "Show in json format")
            (@arg dev: -d --dev [OIF] "Show only route entries with output to the specified interface")
            (about: "Show routes")
        )
        (@subcommand rule =>
            (@arg json: -j --json "Show in json format")
            (about: "Show routes rules")
        )
    )
}
