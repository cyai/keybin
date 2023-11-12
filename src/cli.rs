use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, about, version)]

pub struct Keybinargs {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[clap(about = "Get a secret")]
    Get(Get),
    #[clap(about = "List secrets")]
    List(List),
    #[clap(about = "Store a secret")]
    Store(Store),
    #[clap(about = "Update a secret")]
    Update(Update),
    #[clap(about = "Delete a secret")]
    Delete(Delete),
}

#[derive(Debug, Parser)]
pub struct Get {
    #[clap(short, long, help = "Secret ID")]
    pub secret_id: String,
}

#[derive(Debug, Parser)]
pub struct List {
    #[clap(short, long, help = "Folder")]
    pub folder: Option<String>,
    #[clap(short, long, help = "Tags")]
    pub tags: Option<String>,
    #[clap(short, long, help = "Name Contains")]
    pub name_contains: Option<String>,
    #[clap(short, long, help = "Created At")]
    pub created_at: Option<String>,
    #[clap(short, long, help = "Size")]
    pub size: Option<i32>,
    #[clap(short, long, help = "Order")]
    pub order: Option<String>,
    #[clap(short, long, help = "Order By")]
    pub order_by: Option<String>,
    #[clap(short, long, help = "Last")]
    pub last: Option<String>,
    #[clap(short, long, help = "Include Secrets")]
    pub include_secrets: Option<bool>,
}

#[derive(Debug, Parser)]
pub struct Store {
    #[clap(short, long, help = "Secret name")]
    pub name: String,
    #[clap(short, long, help = "Secret type")]
    pub key_type: Option<String>,
    #[clap(short, long, help = "Secret value")]
    pub value: Option<String>,
    #[clap(short, long, help = "Secret description")]
    pub description: Option<String>,
    #[clap(short, long, help = "Secret tags")]
    pub tags: Option<String>,
}

#[derive(Debug, Parser)]

pub struct Update {
    #[clap(short, long, help = "Secret ID")]
    pub secret_id: String,
    #[clap(short, long, help = "Secret name")]
    pub name: Option<String>,
    #[clap(short, long, help = "Secret folder")]
    pub folder: Option<String>,
    #[clap(short, long, help = "Secret metadata")]
    pub metadata: Option<String>,
    #[clap(short, long, help = "Secret tags")]
    pub tags: Option<String>,
}

#[derive(Debug, Parser)]

pub struct Delete {
    #[clap(short, long, help = "Secret ID")]
    pub secret_id: String,
}
