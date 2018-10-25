extern crate structopt;
extern crate futures;
extern crate iproute2;
extern crate tokio_core;

use futures::Future;
use tokio_core::reactor::Core;

use iproute2::new_connection;

use structopt::StructOpt;

use net_tools::NetRequest;

#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,

    /// Verbose mode (-v, -vv, -vvv, -vvvv)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbosity: u8,

    #[structopt(name = "interface")]
    interface: Option<String>
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "up")]
    Up,
    #[structopt(name = "down")]
    Down
}

fn main() {
    let args = Args::from_args();

    // Create a netlink connection, and a handle to send requests via this connection
    let (connection, handle) = new_connection().unwrap();

    // The connection will run in an event loop
    let core = Core::new().unwrap();
    core.handle().spawn(connection.map_err(|_| ()));

    let handler = NetRequest{
        conn: handle,
        core
    };

    let interface = args.interface.unwrap_or_else(|| String::from(""));

    match args.cmd {
        Some(Command::Up) => handler.up(&interface),
        Some(Command::Down) => handler.down(&interface),
        None => {
            let rx_int = handler.get();

            println!("{:?}", rx_int);
        }
    };
}

