// Copyright (c) 2019 Cloudflare, Inc. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause

use boringtun::device::drop_privileges::drop_privileges;
use boringtun::device::{DeviceConfig, DeviceHandle};
use clap::{Arg, Command};
use daemonize::Daemonize;
use std::fs::File;
use std::os::unix::net::UnixDatagram;
use std::process::exit;
use tracing::Level;

use std::error::Error;

use serde::{Deserialize, Serialize};
use surf::{Url, Request};
use async_std::{task};

#[derive(Debug, Serialize)]
struct RpcRequest<'a> {
    jsonrpc: &'a str,
    id: &'a str,
    method: &'a str,
    params: &'a str,
}

#[derive(Debug, Deserialize)]
struct RpcResponse {
    result: Option<String>,
    error: Option<String>,
}

async fn RpcRequestFunction() -> Result<(), Box<dyn Error>> {
    let url = Url::parse("https://rpc.testnet.near.org")?;
    let jsonrpc = "2.0";
    let id = "dev-1668546365877-28764765342385";
    let method = "query";
    let params = r#"{"request_type": "call_function", "finality": "final", "account_id": "dev-1668546365877-28764765342385", "method_name": "nft_metadata", "args_base64": "e30="}"#;

    let request = Request::new(surf::http::Method::Post, url);
    request.insert_header("Content-Type", "application/json");
    request.body_json(&RpcRequest {
        jsonrpc,
        id,
        method,
        params,
    })?;

      let response = request.await?;
      let body: RpcResponse = response.body_json().await?;

    Ok(())
}

fn main() {
    let matches = Command::new("boringtun")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Vlad Krasnov <vlad@cloudflare.com>")
        .args(&[
            Arg::new("INTERFACE_NAME")
                .required(true)
                .takes_value(true)
                .validator(|tunname| check_tun_name(tunname.to_string()))
                .help("The name of the created interface"),
            Arg::new("foreground")
                .long("foreground")
                .short('f')
                .help("Run and log in the foreground"),
            Arg::new("threads")
                .takes_value(true)
                .long("threads")
                .short('t')
                .env("WG_THREADS")
                .help("Number of OS threads to use")
                .default_value("4"),
            Arg::new("verbosity")
                .takes_value(true)
                .long("verbosity")
                .short('v')
                .env("WG_LOG_LEVEL")
                .possible_values(["error", "info", "debug", "trace"])
                .help("Log verbosity")
                .default_value("error"),
            Arg::new("uapi-fd")
                .long("uapi-fd")
                .env("WG_UAPI_FD")
                .help("File descriptor for the user API")
                .default_value("-1"),
            Arg::new("tun-fd")
                .long("tun-fd")
                .env("WG_TUN_FD")
                .help("File descriptor for an already-existing TUN device")
                .default_value("-1"),
            Arg::new("log")
                .takes_value(true)
                .long("log")
                .short('l')
                .env("WG_LOG_FILE")
                .help("Log file")
                .default_value("/tmp/boringtun.out"),
            Arg::new("disable-drop-privileges")
                .long("disable-drop-privileges")
                .env("WG_SUDO")
                .help("Do not drop sudo privileges"),
            Arg::new("disable-connected-udp")
                .long("disable-connected-udp")
                .help("Disable connected UDP sockets to each peer"),
            #[cfg(target_os = "linux")]
            Arg::new("disable-multi-queue")
                .long("disable-multi-queue")
                .help("Disable using multiple queues for the tunnel interface"),
        ])
        .get_matches();

    let handle = task::spawn(async {RpcRequestFunction()});

    // do some other work here

    let result = task::block_on(handle);

    Ok(())
}

// use std::error::Error;
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize)]
// struct RpcRequest {
//     jsonrpc: String,
//     method: String,
//     params: Vec<String>,
//     id: u32,
// }

// #[derive(Serialize, Deserialize)]
// struct RpcResponse {
//     jsonrpc: String,
//     result: String,
//     id: u32,
// }

// async fn call_rpc() -> Result<(), Box<dyn Error>> {
//     let url = "http://localhost:8080/rpc"; // replace with your RPC server URL
//     let request = RpcRequest {
//         jsonrpc: "2.0".to_owned(),
//         method: "my_rpc_method".to_owned(),
//         params: vec!["param1".to_owned(), "param2".to_owned()],
//         id: 1,
//     };
//     let mut res = surf::post(url)
//         .body(surf::Body::from_json(&request)?)
//         .await?;
//     let response: RpcResponse = res.body_json().await?;
//     println!("RPC response: {:?}", response);
//     Ok(())
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     call_rpc().await?;
//     Ok(())
// }
