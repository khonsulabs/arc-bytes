use khonsu_tools::{
    publish,
    universal::{anyhow, clap::Parser, DefaultConfig},
};

fn main() -> anyhow::Result<()> {
    khonsu_tools::Commands::parse().execute::<Config>()
}

enum Config {}

impl khonsu_tools::Config for Config {
    type Publish = Self;

    type Universal = DefaultConfig;
}

impl publish::Config for Config {
    fn paths() -> Vec<String> {
        vec![String::from("arc-bytes")]
    }
}
