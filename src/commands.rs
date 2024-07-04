use clap::Parser;
use crate::data::DataTable;

/// set_income subcommand
pub fn set_income() {
    todo!();
}

/// set_expense subcommand
pub fn set_expense() {
    todo!();
}

/// get_balance subcommand
pub fn get_balance(_is_listed: &bool) {
    todo!();
}

/// initialize subcommand
/// This function creates a new file named "kakeibo.toml" in the current directory.
pub fn initialize() {
    let data: DataTable = DataTable::default();
    write_data(data, "kakeibo.toml".to_string());
    println!("Initialized kakeibo.toml");
}

fn write_data(data: DataTable, path: String) {
    let string: String = toml::to_string(&data).unwrap();
    if let Err(err) = std::fs::write(path, string) {
        eprintln!("Failed to write data to file: {}", err);
    }
}

/// SET_INCOME constant
const SET_INCOME: &str = "set-income";

/// SET_EXPENSE constant
const SET_EXPENSE: &str = "set-expense";

/// GET_BALANCE constant
const GET_BALANCE: &str = "get-balance";

/// INITIALIZE constant
const INITIALIZE: &str = "init";

/// Parser for kakei command by clap crate
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(help = "What you want to do")]
    pub sub_command: SubCommands,

    #[arg(long, help = "If you want kakei to show as list, use --list")]
    pub list: bool,
}

/// kakei command's SubCommands
#[derive(Debug, Clone, PartialEq)]
pub enum SubCommands {
    SetIncome,
    SetExpense,
    GetBalance,
    Initialize,
}

/// FromStr implement for SubCommands
impl std::str::FromStr for SubCommands {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            SET_INCOME => Ok(Self::SetIncome),
            GET_BALANCE => Ok(Self::GetBalance),
            SET_EXPENSE => Ok(Self::SetExpense),
            INITIALIZE => Ok(Self::Initialize),
            _ => Err(format!("Unknown sub command: {}", s)),
        }
    }
}
