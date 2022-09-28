use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use serde_json::json;

use serde_derive::Deserialize;
//use serde_derive::Serialize;
use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct L1 {
    pub pairs: Vec<L2>,
}
#[derive(Debug, Deserialize)]
pub struct L2 {
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "dexId")]
    pub dex_id: String,
    pub url: String,
    #[serde(rename = "pairAddress")]
    pub pair_address: String,
    #[serde(rename = "priceNative")]
    pub price_native: String,
    #[serde(rename = "priceUsd")]
    pub price_usd: String,
    #[serde(rename = "priceChange")]
    pub price_change: Change,
    pub liquidity: Value,
    pub volume: Value,
    #[serde(rename = "baseToken")]
    pub basetoken: Value,
    //pub fdv: f64,
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Change {
    pub h24: f64,
    pub h6: f64,
    pub h1: f64,
    pub m5: f64,
}

pub async fn run(options: &[CommandDataOption]) -> String {
    let info = options.get(0).unwrap().resolved.as_ref().unwrap();
    let info2 = options.get(1).unwrap().resolved.as_ref().unwrap();
    let info3 = options.get(2).unwrap().resolved.as_ref().unwrap();

    if let CommandDataOptionValue::String(chain) = info {
        if let CommandDataOptionValue::String(address) = info2 {
            if let CommandDataOptionValue::Boolean(invert) = info3 {
                if let Ok(responses) = reqwest::get(format!(
                    "https://api.dexscreener.com/latest/dex/pairs/{}/{}",
                    chain, address
                ))
                .await
                {
                    let status = responses.status().to_string();
                    if status == "200 OK" {
                        if let Ok(v) = responses.json::<L1>().await {
                            let w = v.pairs;
                            if *invert {
                                let price0 = w[0].price_native.parse::<f64>().unwrap();
                                let usd0 = w[0].price_usd.parse::<f64>().unwrap();
                                let usd1 = usd0 / price0;
                                let name1 = "token";
                                let volume = &w[0].volume["h24"].to_string();
                                format!("Price of {}:\n${}\nVolume: ${}", name1, usd1, volume)
                            } else {
                                let usd0 = w[0].price_usd.parse::<f64>().unwrap();
                                let volume = &w[0].volume["h24"];
                                let name0 = "token";
                                let change0 = w[0].price_change.h24;
                                format!(
                                    "Price of {}:\n${}\nVolume: ${}\n24h Change {}%",
                                    name0, usd0, volume, change0
                                )
                            }
                        } else {
                            "Something went wrong with parsing the data".to_string()
                        }
                    } else {
                        "This pair can not be retrieved from dexscreener, make sure you write it down correctly".to_string()
                    }
                } else {
                    "The dexscreener api can not be reached".to_string()
                }
            } else {
                "parsing boolean error".to_string()
            }
        } else {
            "parsing address error".to_string()
        }
    } else {
        "parsing chain error".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("info_coin")
        .description("Get info on a coin by entering their symbol")
        .create_option(|option| {
            option
                .name("chain")
                .description("Select a chain")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice("ethereum", "ethereum")
                .add_string_choice("binance_smart_chain", "bsc")
                .add_string_choice("polygon", "polygon")
                .add_string_choice("avalanche", "avalanche")
                .add_string_choice("fantom", "fantom")
                .add_string_choice("harmony", "harmony")
                .add_string_choice("cronos", "cronos")
                .add_string_choice("osmosis", "aurora")
                .add_string_choice("moonriver", "moonriver")
                .add_string_choice("moonbeam", "moonbeam")
                .add_string_choice("metis", "metis")
                .add_string_choice("arbitrum", "arbitrum")
                .add_string_choice("optimism", "optimism")
                .add_string_choice("dogechain", "dogechain")
                .add_string_choice("heco", "heco")
                .add_string_choice("astar", "astar")
                .add_string_choice("evmos", "evmos")
                .add_string_choice("xdai", "xdai")
        })
        .create_option(|option| {
            option
                .name("address")
                .description("Enter a symbol, or choose a default one!")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("invert")
                .description("turn this on if you want to invert the pair")
                .kind(CommandOptionType::Boolean)
                .required(true)
        })
}
