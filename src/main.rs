extern crate image;
extern crate sheep;
extern crate walkdir;
extern crate rayon;
extern crate clap;

mod pack;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Asset Manager")
                          .version("0.1")
                          .author("humansnotfish")
                          .subcommand(SubCommand::with_name("strip")
                                      .about("takes a directory and strips the images of transparency")
                                      .version("0.1"))
                          .subcommand(SubCommand::with_name("pack")
                                      .about("pack a directory of assets in a sprite sheet")
                                      .version("0.1"))
                          .get_matches();


    // TODO strip read in from a separate directory, like an art directory
    match matches.subcommand() {
        ("pack", _) => pack::pack_tiles(),
        ("strip", _) => panic!("Not implemented"),
        _ => {},
    }
}
