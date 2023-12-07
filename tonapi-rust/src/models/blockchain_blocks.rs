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
pub struct BlockchainBlocks {
    #[serde(rename = "blocks")]
    pub blocks: Vec<crate::models::BlockchainBlock>,
}

impl BlockchainBlocks {
    pub fn new(blocks: Vec<crate::models::BlockchainBlock>) -> BlockchainBlocks {
        BlockchainBlocks {
            blocks,
        }
    }
}


