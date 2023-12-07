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
pub struct BlockchainBlock {
    #[serde(rename = "tx_quantity")]
    pub tx_quantity: i32,
    #[serde(rename = "value_flow")]
    pub value_flow: Box<crate::models::BlockValueFlow>,
    #[serde(rename = "workchain_id")]
    pub workchain_id: i32,
    #[serde(rename = "shard")]
    pub shard: String,
    #[serde(rename = "seqno")]
    pub seqno: i32,
    #[serde(rename = "root_hash")]
    pub root_hash: String,
    #[serde(rename = "file_hash")]
    pub file_hash: String,
    #[serde(rename = "global_id")]
    pub global_id: i32,
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "after_merge")]
    pub after_merge: bool,
    #[serde(rename = "before_split")]
    pub before_split: bool,
    #[serde(rename = "after_split")]
    pub after_split: bool,
    #[serde(rename = "want_split")]
    pub want_split: bool,
    #[serde(rename = "want_merge")]
    pub want_merge: bool,
    #[serde(rename = "key_block")]
    pub key_block: bool,
    #[serde(rename = "gen_utime")]
    pub gen_utime: i64,
    #[serde(rename = "start_lt")]
    pub start_lt: i64,
    #[serde(rename = "end_lt")]
    pub end_lt: i64,
    #[serde(rename = "vert_seqno")]
    pub vert_seqno: i32,
    #[serde(rename = "gen_catchain_seqno")]
    pub gen_catchain_seqno: i32,
    #[serde(rename = "min_ref_mc_seqno")]
    pub min_ref_mc_seqno: i32,
    #[serde(rename = "prev_key_block_seqno")]
    pub prev_key_block_seqno: i32,
    #[serde(rename = "gen_software_version", skip_serializing_if = "Option::is_none")]
    pub gen_software_version: Option<i32>,
    #[serde(rename = "gen_software_capabilities", skip_serializing_if = "Option::is_none")]
    pub gen_software_capabilities: Option<i64>,
    #[serde(rename = "master_ref", skip_serializing_if = "Option::is_none")]
    pub master_ref: Option<String>,
    #[serde(rename = "prev_refs")]
    pub prev_refs: Vec<String>,
    #[serde(rename = "in_msg_descr_length")]
    pub in_msg_descr_length: i64,
    #[serde(rename = "out_msg_descr_length")]
    pub out_msg_descr_length: i64,
    #[serde(rename = "rand_seed")]
    pub rand_seed: String,
    #[serde(rename = "created_by")]
    pub created_by: String,
}

impl BlockchainBlock {
    pub fn new(tx_quantity: i32, value_flow: crate::models::BlockValueFlow, workchain_id: i32, shard: String, seqno: i32, root_hash: String, file_hash: String, global_id: i32, version: i32, after_merge: bool, before_split: bool, after_split: bool, want_split: bool, want_merge: bool, key_block: bool, gen_utime: i64, start_lt: i64, end_lt: i64, vert_seqno: i32, gen_catchain_seqno: i32, min_ref_mc_seqno: i32, prev_key_block_seqno: i32, prev_refs: Vec<String>, in_msg_descr_length: i64, out_msg_descr_length: i64, rand_seed: String, created_by: String) -> BlockchainBlock {
        BlockchainBlock {
            tx_quantity,
            value_flow: Box::new(value_flow),
            workchain_id,
            shard,
            seqno,
            root_hash,
            file_hash,
            global_id,
            version,
            after_merge,
            before_split,
            after_split,
            want_split,
            want_merge,
            key_block,
            gen_utime,
            start_lt,
            end_lt,
            vert_seqno,
            gen_catchain_seqno,
            min_ref_mc_seqno,
            prev_key_block_seqno,
            gen_software_version: None,
            gen_software_capabilities: None,
            master_ref: None,
            prev_refs,
            in_msg_descr_length,
            out_msg_descr_length,
            rand_seed,
            created_by,
        }
    }
}


