use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub servers: BTreeMap<String, Server>,
    pub commands: BTreeMap<String, RemoteCommand>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RemoteCommand {
    pub name: String,
    pub desc: String,
    pub cmd: String,
}