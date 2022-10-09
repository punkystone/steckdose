use clap::Parser;

#[derive(Parser, Debug)]
pub struct Arguments {
    /// Set IP
    #[arg(long)]
    pub ip: String,
    /// Set Action
    #[arg(long, value_enum)]
    pub action: Action,
}
#[allow(non_camel_case_types)]
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Action {
    on,
    off,
    status,
}
