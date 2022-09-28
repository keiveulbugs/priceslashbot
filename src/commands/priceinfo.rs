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

#[derive(poise::ChoiceParameter)]
#[allow(non_camel_case_types)]
pub enum Chain {
    ethereum,
    #[name = "binance_smart_chain"]
    bsc,
    polygon,
    avalanche,
    fantom,
    harmony,
    cronos,
    aurora,
    moonriver,
    moonbeam,
    metis,
    arbitrum,
    optimism,
    dogechain,
    heco,
    astar,
    evmos,
    xdai,
}

/// Get info on a coin by entering their symbol
#[poise::command(slash_command)]
pub async fn info_coin(
    ctx: poise::Context<'_, (), serenity::Error>,
    #[description = "Select a chain"] chain: Chain,
    #[description = "Enter a symbol, or choose a default one!"] address: String,
    #[description = "turn this on if you want to invert the pair"] invert: bool,
) -> Result<(), serenity::Error> {
                let response = if let Ok(responses) = reqwest::get(format!(
                    "https://api.dexscreener.com/latest/dex/pairs/{}/{}",
                    chain, address
                ))
                .await
                {
                    let status = responses.status().to_string();
                    if status == "200 OK" {
                        if let Ok(v) = responses.json::<L1>().await {
                            let w = v.pairs;
                            if invert {
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
                };

    ctx.say(response).await?;
    Ok(())
}
