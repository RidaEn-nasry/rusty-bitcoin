

// block header
pub struct BlockHeader {
    pub version : u32,
    pub prev_block: [u8; 32],
    pub merkle_root: [u8; 32],
    // in unix epoch time
    pub timestamp: u32,
    // difficulty target
    pub bits: u32,
    pub nonce: u32,
}

// transaction 
pub struct Transaction {
    pub version: u32,

    // number of inputs
    pub tx_in_count: u32,
    // inputs
    pub tx_in: Vec<TxIn>,
    pub tx_out_count: u32,
    pub tx_out: Vec<TxOut>,
    pub lock_time: u32,
}


// bitcoin block 
pub struct Block {
    pub magic : u32,
    pub block_size : u32,
    pub block_header : BlockHeader,
    pub tx_count : u32,
}