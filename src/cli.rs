use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, about, version)]

pub struct Keybinargs {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[clap(about = "Get the secret from the vault")]
    Get(Get),
    #[clap(about = "List secrets in the vault")]
    List(List),
    #[clap(about = "Store the secret in the vault")]
    Store(Store),
    #[clap(about = "Update the information of a secret")]
    Update(Update),
    #[clap(about = "Delete the secret from the vault")]
    Delete(Delete),
    #[clap(about = "Get Secret ID from the name")]
    GetId(GetId),
    #[clap(about = "Set Panega Vault API Key")]
    Init(Init),
}

#[derive(Debug, Parser)]
pub struct Get {
    #[clap(short, long, help = "Secret Name", required = true)]
    pub name: String,
}

#[derive(Debug, Parser)]
pub struct List {
    #[clap(short, long, help = "Folder", required = false)]
    pub folder: Option<String>,
    #[clap(short, long, help = "Tags", required = false)]
    pub tags: Option<String>,
    #[clap(short, long, help = "Name Contains", required = false)]
    pub name_contains: Option<String>,
    #[clap(short, long, help = "Created At", required = false)]
    pub created_at: Option<String>,
    #[clap(short, long, help = "Size", required = false)]
    pub size: Option<i32>,
    #[clap(short, long, help = "Order", required = false)]
    pub order: Option<String>,
    #[clap(short, long, help = "Order By", required = false)]
    pub order_by: Option<String>,
    #[clap(short, long, help = "Last", required = false)]
    pub last: Option<String>,
    #[clap(short, long, help = "Include Secrets", required = false)]
    pub include_secrets: Option<bool>,
}

#[derive(Debug, Parser)]
pub struct Store {
    #[clap(short, long, help = "Secret name", required = false)]
    pub name: Option<String>,
    #[clap(short, long, help = "Secret type", required = false)]
    pub key_type: Option<String>,
    #[clap(short, long, help = "Secret value", required = true)]
    pub value: String,
    #[clap(short, long, help = "Secret description", required = false)]
    pub description: Option<String>,
    #[clap(short, long, help = "Secret tags", required = false)]
    pub tags: Option<String>,
}

#[derive(Debug, Parser)]

pub struct Update {
    #[clap(short, long, help = "Secret ID", required = true)]
    pub secret_id: String,
    #[clap(short, long, help = "Secret name", required = false)]
    pub name: Option<String>,
    #[clap(short, long, help = "Secret folder", required = false)]
    pub folder: Option<String>,
    #[clap(short, long, help = "Secret metadata", required = false)]
    pub metadata: Option<String>,
    #[clap(short, long, help = "Secret tags", required = false)]
    pub tags: Option<String>,
}

#[derive(Debug, Parser)]

pub struct Delete {
    #[clap(short, long, help = "Secret ID", required = true)]
    pub secret_id: String,
}


#[derive(Debug, Parser)]
pub struct GetId {
    #[clap(short, long, help = "Secret name", required = true)]
    pub name: String,
}

#[derive(Debug, Parser)]
pub struct Init {
    #[clap(short, long, help = "Panega Vault API Key", required = true)]
    pub key: String,
}