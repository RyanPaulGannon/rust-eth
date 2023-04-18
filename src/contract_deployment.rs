use::std::time::Duration;
use ethers::{
    prelude::{Address, Signer, Middleware, LocalWallet, Provider, TransactionRequest, U256},
    utils::Ganache,
};
use eyre::{ContextCompat, Result};
use hex::ToHex;

#[tokio::main]
async fn main() -> Result<()> {
    let mnemonic = "glory attend earn text absorb narrow visit strong runway outside velvet";
    let ganache = Ganache::new().mnemonic(mnemonic).spawn();
    println!("Ganache is running on: {}", ganache.endpoint());

    let wallet: LocalWallet = ganache.keys()[0].clone().into();
    let first_address = wallet.address();
    println!("First address: {}", first_address.encode_hex::<String>());
}