#[macro_use]
extern crate structopt;

extern crate futures;
extern crate iproute2;
extern crate tokio_core;

use futures::Future;
use tokio_core::reactor::Core;

use iproute2::new_connection;
use std::thread::spawn;

use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(subcommand)]
    pub cmd: Option<Command>,

    /// Verbose mode (-v, -vv, -vvv, -vvvv)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: u8,

    #[structopt(name = "interface", parse(from_os_str))]
    pub interface: PathBuf
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "up")]
    Up,
    #[structopt(name = "down")]
    Down
}

fn main() {
    let args = Args::from_args();

    let link_name = args.interface;

    let (connection, handle) = new_connection().unwrap();
    spawn(move || Core::new().unwrap().run(connection));

    // Get the list of links
    let links = handle.link().get().execute().wait().unwrap();

    // Find the link with the name provided as argument, and delete it
    for link in links {
        if link.name().unwrap() == link_name.to_string_lossy() {
            handle.link().del(link.index()).execute().wait().unwrap();
        }
    }
}
