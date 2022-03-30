extern crate core;
use clap::{Parser, Subcommand};
use reqwest::Client;
use serde_json;
use mint_client::ResBody;

#[derive(Parser)]
#[clap(name = "MiniMint CLI", about = "CLI to use the MiniMint RPC-Client (clientd)")]
struct Cli {
    #[clap(subcommand)]
    command : Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Clients holdings (total, coins, pending)
    Info {
        /// Format JSON
        #[clap(takes_value = false, long="raw")]
        raw : bool,
    },
    /// Clients pending coins
    Pending {
        /// Format JSON
        #[clap(takes_value = false, long="raw")]
        raw : bool,
    },
    /// The spend subcommand allows to send tokens to another client. This will select the smallest possible set of the client's coins that represents a given amount.
    #[clap(arg_required_else_help = true)]
    Spend {
        /// The amount of coins to be spend in msat if not set to sat
        amount : u64,
        /// Format JSON
        #[clap(takes_value = false, long="raw")]
        raw : bool,
    },
    /// Reissue coins to claim them and avoid double spends
    #[clap(arg_required_else_help = true)]
    Reissue {
        /// The base64 encoded coins
        coins : String,
        /// Format JSON
        #[clap(takes_value = false, long="raw")]
        raw : bool,
        #[clap(takes_value = false, long="silent")]
        silent : bool,
    },
    Events {
        #[clap(takes_value = false, long="raw")]
        raw : bool,
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let (res, raw) = match &args.command {
        Commands::Info { raw } => {
            (call_clientd("info", "").await, raw)
        },
        Commands::Pending {raw} => {
            (call_clientd("pending", "").await, raw)
        },
        Commands::Spend {amount, raw} => {
            (call_clientd("spend", amount).await, raw)
        },
        Commands::Reissue {coins, raw, silent} => {
            if *silent {
                (call_clientd("reissue", coins).await, raw)
            } else {
                (call_clientd("reissue_validate", coins).await, raw)
            }
        },
        Commands::Events { raw} => {
            (call_clientd("events", "").await, raw)
        },
    };
    match res {
        Ok(res) => print_res(res, *raw),
        Err(_) => (),
    }
}

async fn call_clientd<T : serde::ser::Serialize + ?Sized>(query : &str, json : &T) -> Result<ResBody, reqwest::Error>{
    let res = Client::new()
        .post(format!("{}{}", "http://127.0.0.1:8080/", query))
        .json(json)
        .send()
        .await?;
    let res : ResBody = res.json().await?;
    Ok(res)
}

fn print_res(res : ResBody, raw : bool) {
    if raw {
        //print raw json-string
        //unwrap is ok, since ResBody is always a valid data structure
        println!("{}", serde_json::to_string(&res).unwrap());
    } else {
        //print pretty json-string
        //unwrap is ok, since ResBody is always a valid data structure
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
    }
}

// Good Error Handling
//whats ?Trait
//why use dyn instead of traitbounds (rust book)
// tf is this : impl<T> const From<!> for T