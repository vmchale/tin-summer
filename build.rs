#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use]
extern crate clap;

use clap::Shell;
use clap::App;

fn main() {

    // load configuration
    #[cfg(feature = "english")]
    let yaml = load_yaml!("src/cli/options-en.yml");
    #[cfg(feature = "francais")]
    let yaml = load_yaml!("src/cli/options-fr.yml");
    #[cfg(feature = "deutsch")]
    let yaml = load_yaml!("src/cli/options-de.yml");
    let mut app = App::from_yaml(yaml).version(crate_version!());

    // generate bash completions if desired
    #[cfg(feature = "bash")] app.gen_completions("sn", Shell::Bash, env!("BASH_COMPLETIONS_DIR"));

    // generate fish completions if desired
    let mut app_snd = App::from_yaml(yaml).version(crate_version!());
    #[cfg(feature = "fish")]
    app_snd.gen_completions("sn", Shell::Fish, env!("FISH_COMPLETIONS_DIR"));

}
