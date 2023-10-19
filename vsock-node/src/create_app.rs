use log::error;

pub trait ExitGracefully<T, E> {
    fn ok_or_exit(self, msg: &str) -> T;
}

impl<T, E> ExitGracefully<T, E> for Result<T, E>
where
    E: std::fmt::Debug,
{
    fn ok_or_exit(self, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                error!("{:?}: {}", err, msg);
                std::process::exit(1);
            }
        }
    }
}

#[macro_export]
macro_rules! create_app {
    () => {
        App::new("net example")
            .about("example for vsock server and client communication.")
            .setting(AppSettings::ArgRequiredElseHelp)
            .version(env!("CARGO_PKG_VERSION"))
            .subcommand(
                Command::new("tcp_to_vsock_server")
                    .about("Listen on a give tcp(host+port) and send by vsock(cid+port)")
                    .arg(
                        Arg::new("cid")
                            .long("cid")
                            .help("cid")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::new("port")
                            .long("port")
                            .help("port")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::new("tcpPort")
                            .long("tcpPort")
                            .help("tcpPort")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::new("host")
                            .long("host")
                            .help("host")
                            .takes_value(true)
                            .required(true),
                    ),
            )
            .subcommand(
                Command::new("vsock_client")
                    .about("connect to a given cid and port")
                    .arg(
                        Arg::new("port")
                            .long("port")
                            .help("port")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::new("cid")
                            .long("cid")
                            .help("cid")
                            .takes_value(true)
                            .required(true),
                    ),
            )
            .subcommand(
                Command::new("vsock_to_tcp_server")
                    .about("listen a given port and send by tcp(host+port)")
                    .arg(
                        Arg::new("tcpPort")
                            .long("tcpPort")
                            .help("tcpPort")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::new("host")
                            .long("host")
                            .help("host")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::new("port")
                            .long("port")
                            .help("port")
                            .takes_value(true)
                            .required(true),
                    ),
            )
            .subcommand(
                Command::new("tcp_to_tcp_server")
                    .about("listen a given port and send by tcp(host+port)")
                    .arg(
                        Arg::new("tcpPort")
                            .long("tcpPort")
                            .help("tcpPort")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::new("host")
                            .long("host")
                            .help("host")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::new("toHost")
                            .long("toHost")
                            .help("toHost")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::new("toPort")
                            .long("toPort")
                            .help("toPort")
                            .takes_value(true)
                            .required(true),
                    ),
            )
            .subcommand(
                Command::new("tcp_client")
                    .about("connect to a given cid and port")
                    .arg(
                        Arg::new("tcpPort")
                            .long("tcpPort")
                            .help("tcpPort")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::new("host")
                            .long("host")
                            .help("host")
                            .takes_value(true)
                            .required(true),
                    ),
            )
    };
}
