use clap::{Parser, Subcommand};
use reqwest::{Body, Client};
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
        #[clap(takes_value = false, short='p')]
        pretty : bool,
    },
    /// Clients pending coins
    Pending {
        /// Format JSON
        #[clap(takes_value = false, short='p')]
        pretty : bool,
    },
    /// The spend subcommand allows to send tokens to another client. This will select the smallest possible set of the client's coins that represents a given amount.
    #[clap(arg_required_else_help = true)]
    Spend {
        /// The amount of coins to be spend in msat if not set to sat
        amount : u64,
        /// Format JSON
        #[clap(takes_value = false, short='p')]
        pretty : bool,
    },
    /// Reissue coins to claim them and avoid double spends
    #[clap(arg_required_else_help = true)]
    Reissue {
        /// The base64 encoded coins
        coins : String,
        /// Format JSON
        #[clap(takes_value = false, short='p')]
        pretty : bool,
        #[clap(takes_value = false, short='v')]
        validate : bool,
    },
    Events {
        #[clap(takes_value = false, short='p')]
        pretty : bool,
    }
}
#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Info { pretty } => {
            info(*pretty).await;
        },
        Commands::Pending {pretty} => {
            pending(*pretty).await;
        },
        Commands::Spend {amount, pretty} => {
            spend(*pretty, *amount).await;
        },
        Commands::Reissue {coins, pretty, validate} => {
            reissue(*pretty, *validate, coins).await;
        },
        Commands::Events { pretty} => {
            event(*pretty).await;
        },
    }
}

async fn info(p: bool) {
    let res = Client::new()
        .post("http://127.0.0.1:8080/info") //?
        .send()
        .await.unwrap();
    let res : ResBody = res.json().await.unwrap();
    if p {
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
    }else {
        println!("{}", serde_json::to_string(&res).unwrap());
    }
}
async fn pending(p: bool) {
    let res = Client::new()
        .post("http://127.0.0.1:8080/pending") //?
        .send()
        .await.unwrap();
    let res : ResBody = res.json().await.unwrap();
    if p {
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
    }else {
        println!("{}", serde_json::to_string(&res).unwrap());
    }
}

async fn spend(p: bool, coins: u64) {
    let res = Client::new()
        .post("http://127.0.0.1:8080/spend")
        .json(&coins)
        .send()
        .await.unwrap();
    let res : ResBody = res.json().await.unwrap();
    if p {
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
    }else {
        println!("{}", serde_json::to_string(&res).unwrap());
    }
}

async fn reissue(p: bool, v: bool, coins : &String){
    let url = if v { "http://127.0.0.1:8080/reissue_validate"} else { "http://127.0.0.1:8080/reissue" };
    let res = Client::new()
        .post(url)
        .json(&coins)
        .send()
        .await.unwrap();
    let res : ResBody = res.json().await.unwrap();
    if p {
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
    }else {
        println!("{}", serde_json::to_string(&res).unwrap());
    }
}

async fn event(p: bool) {
    let res = Client::new()
        .post("http://127.0.0.1:8080/events")
        .send()
        .await.unwrap();
    let res : ResBody = res.json().await.unwrap();
    if p {
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
    }else {
        println!("{}", serde_json::to_string(&res).unwrap());
    }
}

// Good Error Handling
// Where to store config like host:port
// Format the json properly and not just puke it to stdout
//instead of -p for pretty do --raw for unpretty so pretty is standard