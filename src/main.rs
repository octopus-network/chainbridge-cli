use structopt::StructOpt;
pub type ResourceId = [u8; 32];

#[derive(Debug, StructOpt)]
pub struct Gid {
    /// chain id
    pub chain_id: Option<u8>,
    /// token name
    pub token_contract_address: Option<String>,
}

impl Gid {

    pub fn derive_resource_id(&self, chain: u8, id: &[u8]) -> ResourceId {
        let mut r_id: ResourceId = [0; 32];
        r_id[31] = chain; // last byte is chain id
        let range = if id.len() > 31 { 31 } else { id.len() }; // Use at most 31 bytes
        for i in 0..range {
            r_id[30 - i] = id[range - 1 - i]; // Ensure left padding for eth compatibilit
        }

        r_id
    }

    fn run(&self) -> anyhow::Result<()> {
        println!("generate resource id: Start!");
        let chain_id = self.chain_id.ok_or(anyhow::anyhow!("must be give chain id"))?;
        let token_contract_address = self.token_contract_address.clone().ok_or(anyhow::anyhow!("must be give token_contract_address"))?;
        if !token_contract_address.starts_with("0x") {
            return Err(anyhow::anyhow!("invalid token_contract_address"));
        }
        let token_contract_address = hex::decode(token_contract_address.trim_start_matches("0x")).map_err(|_| anyhow::anyhow!("token contract address hex decode failed"))?;
        let result = self.derive_resource_id(chain_id, &token_contract_address);
        let hex_encode = hex::encode(result);
        println!("chain id: {}, token_contract_address: {:?}, generate resource id: 0x{}", chain_id, self.token_contract_address, hex_encode);
    
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// generate reousrce id
    #[structopt(name = "generate-resource-id")]
    Gid(Gid),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "chainbridge-cli")]
pub struct AppArguments {
    #[structopt(subcommand)]
    pub command: Command,
}

impl AppArguments {
    fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Command::Gid(value) => value.run(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let opt = AppArguments::from_args();
    opt.run()?;

    Ok(())
}
