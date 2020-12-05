use blockchain::Blockchain;
use rustcoinlib::*;
mod blockchain ;

fn main() {

    let mut blockchain = Blockchain::new() ;

    let first_block = Block::new(
                            vec![Transaction{
                                sender:String::from("Ryan"),
                                reciever:String::from("Dan"),
                                amount: 2000.0,
                            }]
                        );

  //  println!("{:#?}",first_block);

    let second_block = Block::new(
                            vec![Transaction{
                                           sender:String::from("Sam"),
                                           reciever:String::from("Dan"),
                                           amount:1000.0,
                                        }]
                                    );
                                        
    //println!("{:#?}",second_block);

     let third_block = Block::new(
        vec![Transaction{
            sender:String::from("Ryan"),
            reciever:String::from("Julie"),
            amount: 2000.0,
        }]
    );

//println!("{:#?}",first_block);

let fourth_block = Block::new(
        vec![Transaction{
                       sender:String::from("Sam"),
                       reciever:String::from("Kyle"),
                       amount:1000.0  
            }],
         );
//println!("{:#?}",second_block);

     blockchain.add_block(first_block);
     blockchain.add_block(second_block);

     blockchain.add_block(third_block);
     blockchain.add_block(fourth_block);

     println!("{:#?}",blockchain);
     
}
