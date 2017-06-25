#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

pub static VERSION: &'static str = concat!(
    include_str!(concat!(env!("OUT_DIR"), "/git_commit_hash.txt")),
    " v",
    env!("CARGO_PKG_VERSION"));

fn main() {
    let matches = App::new("bindery")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(VERSION)
        .about("a place for making books")
        .author(crate_authors!())
        .subcommand(SubCommand::with_name("analyze")
                    .about("parse puzzle hunt graph and display findings")
                    .arg(Arg::with_name("debug")
                         .help("print additional output to stdout for development")
                         .long("debug"))
                    .arg(Arg::with_name("hints")
                         .help("include hints in the image output (requires --image)")
                         .long("hints")
                         .requires("image"))
                    .arg(Arg::with_name("image")
                         .help("create a visual version of the puzzle hunt with graphviz dot")
                         .long("image")
                         .takes_value(true)))
        .subcommand(SubCommand::with_name("render")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .about("transform a puzzle hunt graph into the target architecture")
                    .subcommand(SubCommand::with_name("js")
                                .about("target js backend"))
                    .subcommand(SubCommand::with_name("arduino")
                                .about("target arduino backend")))
        .subcommand(SubCommand::with_name("build")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .about("produce executable puzzle hunt on the target architecture")
                    .subcommand(SubCommand::with_name("js")
                                .about("target js backend"))
                    .subcommand(SubCommand::with_name("arduino")
                                .about("target arduino backend")))
        .get_matches();

    match matches.subcommand() {
        ("analyze", _) => {
            println!("analyze not yet implemented")
        },
        ("render", Some(render_matches)) => {
            match render_matches.subcommand() {
                ("arduino", _) => println!("render arduino not yet implemented"),
                ("js", _) => println!("render js not yet implemented"),
                _ => unreachable!(),
            }
        },
        ("build", Some(build_matches)) => {
            match build_matches.subcommand() {
                ("arduino", _) => println!("build arduino not yet implemented"),
                ("js", _) => println!("build js not yet implemented"),
                _ => unreachable!(),
            }
        },
        ("", None) => {
            println!("no subcommand specified. see `bindery help` for more information.")
        }
        _ => unreachable!(),
    }
}
