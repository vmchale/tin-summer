#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use] extern crate clap;

use clap::Shell;
use clap::App;

fn main() {

    // load configuration
    let yaml = load_yaml!("src/options-en.yml");
    let mut app = App::from_yaml(yaml).version(crate_version!());

    // generate bash completions if desired
    #[cfg(feature = "bash")]
    app.gen_completions("sniff",          // We need to specify the bin name manually
                        Shell::Bash,      // Then say which shell to build completions for
                        env!("BASH_COMPLETIONS_DIR")); // Then say where write the completions to

    // generate fish completions if desired
    let mut app_snd = App::from_yaml(yaml).version(crate_version!());
    #[cfg(feature = "fish")]
    app_snd.gen_completions("sniff",          // We need to specify the bin name manually
                        Shell::Fish,      // Then say which shell to build completions for
                        env!("FISH_COMPLETIONS_DIR")); // Then say where write the completions to

}
