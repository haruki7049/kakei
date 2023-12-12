use clap::Parser;
use serde::{Deserialize, Serialize};

fn main() {
    let args: Args = Args::parse();
    let is_listed: bool = args.list;

    match args.sub_command {
        SubCommands::SetIncome => set_income(),
        SubCommands::SetExpense => set_expense(),
        SubCommands::GetBalance => get_balance(&is_listed),
    }
}

fn set_income() {
    println!("set-income command typed");
}

fn set_expense() {
    println!("set-expense command typed");
}

fn get_balance(_is_listed: &bool) {
    println!("get-balance command typed");
}

const SET_INCOME: &str = "set-income";
const SET_EXPENSE: &str = "set-expense";
const GET_BALANCE: &str = "get-balance";

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(help = "What you want to do")]
    pub sub_command: SubCommands,

    #[arg(long, help = "If you want kakei to show as list, use --list")]
    pub list: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SubCommands {
    SetIncome,
    SetExpense,
    GetBalance,
}

impl std::str::FromStr for SubCommands {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            SET_INCOME => Ok(Self::SetIncome),
            GET_BALANCE => Ok(Self::GetBalance),
            SET_EXPENSE => Ok(Self::SetExpense),
            _ => Err(format!("Unknown sub command: {}", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Balance {
    expenses: Vec<Price>,
    incomes: Vec<Price>,
}

impl Balance {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            expenses: Vec::new(),
            incomes: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Price {
    price: f64,
    unit: Unit,
}

#[derive(Serialize, Deserialize, Debug)]
enum Unit {
    Yen,
}
