use structopt::StructOpt;
pub type ResourceId = [u8; 32];

#[derive(Debug, StructOpt)]
pub struct Gid {
    // Note: Chain ID is 0 indicating this is native to another chain
    /// chain id
    pub chain_id: Option<u8>,
    /// token name
    pub token_name: Option<String>,
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
        println!("👉🏼👉🏼generate resource id: Start!");
        let chain_id = self.chain_id.ok_or(anyhow::anyhow!("must be give chain id"))?;
        let token_name = self.token_name.clone().ok_or(anyhow::anyhow!("must be give token_name"))?;

        let token_name_hash = sp_io::hashing::blake2_128(token_name.as_bytes());
        let result = self.derive_resource_id(chain_id, &token_name_hash);
        let hex_encode = hex::encode(result);
        println!("🎈🎈chain id: {}, token_name: {}, generate resource id: 0x{}", chain_id, self.token_name.as_ref().unwrap(), hex_encode);
        println!("🌈🌈generate resource id: Successfull!🌈🌈");
    
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct Str2Hex {
    /// hex String content
   content: Option<String>,
}

impl Str2Hex {
    pub fn run(&self) -> anyhow::Result<()> {
        println!("👉🏼👉🏼hex content: Start!");
        let content = self.content.clone().ok_or(anyhow::anyhow!("Empty Content"))?;
        let hex_content = hex::encode(content.as_bytes());
        println!("🎈🎈content: {},hex content : 0x{}", content, hex_content);
        println!("🌈🌈Content to hex : Successfull!🌈🌈");
    
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// generate reousrce id
    #[structopt(name = "generate-resource-id")]
    Gid(Gid),
    /// encode content to hex
    #[structopt(name = "str2hex")]
    Str2Hex(Str2Hex),
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
            Command::Str2Hex(value) => value.run(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let opt = AppArguments::from_args();
    opt.run()?;

    Ok(())
}
