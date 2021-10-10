use bevy::prelude::*;
use clap::Clap;

pub struct CliPlugin;
impl Plugin for CliPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Opts::parse());
    }
}

#[derive(Clap)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcommand: Option<SubCommand>,
}

#[derive(Clap)]
pub enum SubCommand {
    Connect(ConnectSubcommand),
    Host(HostSubcommand),
}

#[derive(Clap)]
pub struct ConnectSubcommand {}

#[derive(Clap)]
pub struct HostSubcommand {}
