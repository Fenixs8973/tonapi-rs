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
pub struct GetRawShardInfo200Response {
    #[serde(rename = "id")]
    pub id: Box<crate::models::BlockRaw>,
    #[serde(rename = "shardblk")]
    pub shardblk: Box<crate::models::BlockRaw>,
    #[serde(rename = "shard_proof")]
    pub shard_proof: String,
    #[serde(rename = "shard_descr")]
    pub shard_descr: String,
}

impl GetRawShardInfo200Response {
    pub fn new(id: crate::models::BlockRaw, shardblk: crate::models::BlockRaw, shard_proof: String, shard_descr: String) -> GetRawShardInfo200Response {
        GetRawShardInfo200Response {
            id: Box::new(id),
            shardblk: Box::new(shardblk),
            shard_proof,
            shard_descr,
        }
    }
}


