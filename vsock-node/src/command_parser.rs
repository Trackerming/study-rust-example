use clap::ArgMatches;

#[derive(Debug, Clone)]
pub struct ServerArgs {
    pub port: u32,
}

fn parse_port(args: &ArgMatches) -> Result<u32, String> {
    let port = args
        .value_of("port")
        .ok_or_else(|| "couldn't find port argument.")?;
    port.parse()
        .map_err(|_err| "port is not a number".to_string())
}

fn parse_cid_client(args: &ArgMatches) -> Result<u32, String> {
    let cid = args
        .value_of("cid")
        .ok_or_else(|| format!("could not find cid argument."))?;
    cid.parse()
        .map_err(|_err| "cid is not a number".to_string())
}

impl ServerArgs {
    pub fn new_with(args: &ArgMatches) -> Result<Self, String> {
        Ok(ServerArgs {
            port: parse_port(args)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ClientArgs {
    pub cid: u32,
    pub port: u32,
}

impl ClientArgs {
    pub fn new_with(args: &ArgMatches) -> Result<Self, String> {
        Ok(ClientArgs {
            cid: parse_cid_client(args)?,
            port: parse_port(args)?,
        })
    }
}
