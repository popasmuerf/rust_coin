mod block; 
pub use block::Block ;
pub mod transaction ;
pub use transaction::Transaction ;
use std::time::{SystemTime, UNIX_EPOCH};



pub fn now() -> u64{
    let start = SystemTime::now() ;
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");

        since_the_epoch.as_secs()
}

pub fn calculate_hash(pre_hash:&String,transactions:&Vec<Transaction>,timestamp:&u64)-> String{
    let mut bytes = vec![] ;

    //let mut foo = &timestamp.to_ne_bytes();

    bytes.extend(&timestamp.to_ne_bytes());
    bytes.extend(transactions
                 .iter()
                 .flat_map(|transaction| transaction.bytes())
                 .collect::<Vec<u8>>() );
    bytes.extend(pre_hash.as_bytes()) ;

    crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
}
