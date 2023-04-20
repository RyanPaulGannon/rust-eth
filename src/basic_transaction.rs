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

    let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));

    let first_balance = provider.get_balance(first_address, None).await?;
    println!("Wallet of the first address balance: {}", first_balance);

    let other_hex_address = "0x9B707cB77D0c2995Bd6D3B593ecabd39614acc88";
    let other_address = "0x9B707cB77D0c2995Bd6D3B593ecabd39614acc88".parse::<Address>()?;
    let other_balance = provider.get_balance(other_address, None).await?;
    println!("Balance for address: {}, is: {}", other_address, other_balance);

    let tx = TransactionRequest::pay(other_address, U256::from(1000u64)).from(first_address);

    let receipt = provider
       .send_transaction(tx, None)
       .await?
       .log_msg("Pending transfer")
       .confirmations(1)
       .await?
       .context("Missing receipt")?;

    println!("TX mined in block: {}", receipt.block_number.context("Missing block number")?);

    println!("Balance for address: {}, is: {}", other_hex_address, provider.get_balance(other_address, None).await?);
    Ok(())
}