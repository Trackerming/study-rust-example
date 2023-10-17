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
                Command::new("vsock_sever")
                    .about("Listen on a give port")
                    .arg(
                        Arg::new("port")
                            .long("port")
                            .help("port")
                            .takes_value("port")
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
                Command::new("tcp_server")
                    .about("listen a given cid and port")
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
