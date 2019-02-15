extern crate iron;
extern crate router;
extern crate rand;
extern crate secp256k1;
extern crate bitcoin;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;
use secp256k1::{Secp256k1, SecretKey, PublicKey};

use bitcoin::network::constants::Network;
use bitcoin::util::address::Payload;
use bitcoin::util::address::Address;

fn handler_two(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();

    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    println!("{:?}", payload);

    Ok(Response::with((status::Ok, "")))
}

fn main() {
    //let mut router = Router::new();           // Alternative syntax:
    //router.get("/", handler, "index");        // let router = router!(index: get "/" => handler,
    //router.get("/:query", handler, "query");  //                      query: get "/:query" => handler);
    //router.post("/testpost", handler_two, "post");

    //Iron::new(router).http("localhost:3000").unwrap();

    //fn handler(req: &mut Request) -> IronResult<Response> {
    //    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    //    Ok(Response::with((status::Ok, *query)))
    //}
 
    let network = Network::Bitcoin;
 
    // Generate random key pair
    let s = Secp256k1::new();
    //let (secret_key, public_key) = s.generate_keypair(&mut thread_rng());
    let secret_key = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");
    let public_key = PublicKey::from_secret_key(&s, &secret_key);
 
    // Generate pay-to-pubkey address
    let address = Address::p2pk(&public_key, network);
    println!("{}", address);
 
    // Check address payload is public key given
    assert_eq!(address.payload, Payload::Pubkey(public_key));
 
    // Check address can be unlocked by secret_key
    assert_eq!(address.payload, Payload::Pubkey(PublicKey::from_secret_key(&s, &secret_key)));
}