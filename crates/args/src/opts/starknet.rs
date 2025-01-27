use std::str::FromStr;

use clap::Args;
use reqwest::Url;
use starknet::core::types::FieldElement;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;

#[derive(Debug, Clone, Args)]
#[command(next_help_heading = "Starknet options")]
pub struct StarknetOptions {
    /// The Starknet JSON-RPC endpoint.
    #[arg(long = "rpc", value_name = "URL", env = "STARKNET_RPC_URL")]
    #[arg(default_value = "http://localhost:5050/")]
    pub rpc_url: Url,
}

impl StarknetOptions {
    pub fn provider(&self) -> JsonRpcClient<HttpTransport> {
        JsonRpcClient::new(HttpTransport::new(self.rpc_url.clone()))
    }
}

#[derive(Debug, Clone, strum::Display, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum ChainId {
    Mainnet,
    Sepolia,
}

impl ChainId {
    /// `SN_MAIN` in ASCII
    pub const SN_MAIN: FieldElement = FieldElement::from_mont([
        0xf596341657d6d657,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0x6f9757bd5443bc6,
    ]);

    /// `SN_SEPOLIA` in ASCII
    pub const SN_SEPOLIA: FieldElement = FieldElement::from_mont([
        0x159755f62c97a933,
        0xfffffffffff59634,
        0xffffffffffffffff,
        0x70cb558f6123c62,
    ]);

    pub fn id(&self) -> FieldElement {
        match self {
            Self::Mainnet => Self::SN_MAIN,
            Self::Sepolia => Self::SN_SEPOLIA,
        }
    }

    pub fn options<'a>() -> &'a [&'static str] {
        &["Mainnet", "Sepolia"]
    }
}

impl TryFrom<FieldElement> for ChainId {
    type Error = InvalidChain;
    fn try_from(value: FieldElement) -> Result<Self, Self::Error> {
        if value == Self::SN_MAIN {
            Ok(Self::Mainnet)
        } else if value == Self::SN_SEPOLIA {
            Ok(Self::Sepolia)
        } else {
            Err(InvalidChain(format!("{value:#x}")))
        }
    }
}

impl FromStr for ChainId {
    type Err = InvalidChain;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mainnet" => Ok(Self::Mainnet),
            "sepolia" => Ok(Self::Sepolia),
            _ => Err(InvalidChain(s.to_string())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("invalid chain: {0}")]
pub struct InvalidChain(String);
