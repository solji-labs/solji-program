use anchor_lang::prelude::*;


#[account]
#[derive(Default,Debug)]
pub struct IncenseNFT {}


impl IncenseNFT {
    pub const SEED_PREFIX: &'static str = "IncenseNFT";
    pub const TOKEN_DECIMALS: u8 = 0;
    pub const TOKEN_NAME: &'static str = "IncenseNFT";
    pub const TOKEN_SYMBOL: &'static str = "INCENSE";
    pub const TOKEN_URL: &'static str = "https://note-public-img.oss-cn-beijing.aliyuncs.com/nya/nya.json"; // TODO 通过IPFS生成
}


