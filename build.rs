#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use]
extern crate clap;
extern crate cli_setup;

use clap::App;
use clap::Shell;
use cli_setup::*;

pub const MAN_SN: &'static str = include_str!("man/tin-summer.1");

fn main() {
    setup_thefuck();

    setup_manpages(MAN_SN, "tin-summer");

    // load configuration
    #[cfg(feature = "english")]
    let yaml = load_yaml!("src/cli/options-en.yml");
    #[cfg(feature = "francais")]
    let yaml = load_yaml!("src/cli/options-fr.yml");
    #[cfg(feature = "deutsch")]
    let yaml = load_yaml!("src/cli/options-de.yml");
    let mut app = App::from_yaml(yaml).version(crate_version!());

    // generate bash completions if desired
    #[cfg(feature = "bash")]
    app.gen_completions("sn", Shell::Bash, env!("BASH_COMPLETIONS_DIR"));

    // generate fish completions if desired
    let mut app_snd = App::from_yaml(yaml).version(crate_version!());
    #[cfg(feature = "fish")]
    app_snd.gen_completions("sn", Shell::Fish, env!("FISH_COMPLETIONS_DIR"));
}
