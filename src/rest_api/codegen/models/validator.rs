use serde::{Deserialize, Serialize};
use tonlib::address::TonAddress;

/*
 * REST api to TON blockchain explorer
 *
 * Provide access to indexed TON blockchain
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@tonkeeper.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Validator {
    #[serde(rename = "address")]
    pub address: TonAddress,
    #[serde(rename = "adnl_address")]
    pub adnl_address: String,
    #[serde(rename = "stake")]
    pub stake: i64,
    #[serde(rename = "max_factor")]
    pub max_factor: i64,
}

impl Validator {
    pub fn new(
        address: TonAddress,
        adnl_address: String,
        stake: i64,
        max_factor: i64,
    ) -> Validator {
        Validator {
            address,
            adnl_address,
            stake,
            max_factor,
        }
    }
}
