use clap::{App, AppSettings, Arg, Command};
use vsock_node::command_parser::{ClientArgs, ServerArgs, TcpArgs};
use vsock_node::{client, create_app, tcp_client, tcp_to_vsock, vsock_to_tcp};

fn main() {
    let app = create_app!();
    let args = app.get_matches();
    match args.subcommand() {
        Some(("vsock_to_tcp_sever", args)) => {
            let server_ags = ServerArgs::new_with(args).unwrap();
            let tcp_args = TcpArgs::new_with(args).unwrap();
            vsock_to_tcp(server_ags, tcp_args).unwrap();
        }
        Some(("vsock_client", args)) => {
            let client_ags = ClientArgs::new_with(args).unwrap();
            client(client_ags).unwrap();
        }
        Some(("tcp_to_vsock_server", args)) => {
            let tcp_args = TcpArgs::new_with(args).unwrap();
            let client_ags = ClientArgs::new_with(args).unwrap();
            tcp_to_vsock(tcp_args, client_ags).unwrap();
        }
        Some(("tcp_client", args)) => {
            let tcp_ags = TcpArgs::new_with(args).unwrap();
            tcp_client(tcp_ags).unwrap();
        }
        Some(_) | None => (),
    }
}
