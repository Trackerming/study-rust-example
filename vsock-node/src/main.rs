use clap::{App, AppSettings, Arg, Command};
use vsock_node::command_parser::{ClientArgs, ServerArgs, TcpArgs};
use vsock_node::{client, create_app, server, tcp_client, tcp_server};

fn main() {
    let app = create_app!();
    let args = app.get_matches();
    match args.subcommand() {
        Some(("vsock_sever", args)) => {
            let server_ags = ServerArgs::new_with(args).unwrap();
            server(server_ags).unwrap();
        }
        Some(("vsock_client", args)) => {
            let client_ags = ClientArgs::new_with(args).unwrap();
            client(client_ags).unwrap();
        }
        Some(("tcp_server", args)) => {
            let tcp_ags = TcpArgs::new_with(args).unwrap();
            tcp_server(tcp_ags).unwrap();
        }
        Some(("tcp_client", args)) => {
            let tcp_ags = TcpArgs::new_with(args).unwrap();
            tcp_client(tcp_ags).unwrap();
        }
        Some(_) | None => (),
    }
}
