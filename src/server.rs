//! server of Blockchain

use super::*;
use crate::block::*;
use crate::transaction::*;
use crate::utxoset::*;
use bincode::{deserialize, serialize};
use failure::format_err;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::*;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Message {
    Addr(Vec<String>),
    Version(Versionmsg),
    Tx(Txmsg),
    GetData(GetDatamsg),
    GetBlock(GetBlocksmsg),
    Inv(Invmsg),
    Block(Blockmsg),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Blockmsg {
    addr_from: String,
    block: Block,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GetBlocksmsg {
    addr_from: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GetDatamsg {
    addr_from: String,
    kind: String,
    id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Invmsg {
    addr_from: String,
    kind: String,
    items: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Txmsg {
    addr_from: String,
    transaction: Transaction,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Versionmsg {
    addr_from: String,
    version: i32,
    best_height: i32,
}
