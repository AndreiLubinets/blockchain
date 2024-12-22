use alloy::primitives::Address;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'a', long = "addr")]
    pub contract_address: Option<Address>,
}
