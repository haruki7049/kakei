use clap::Parser;
use kakei::commands::{get_balance, set_expense, set_income, Args, SubCommands};

fn main() {
    let args: Args = Args::parse();
    let is_listed: bool = args.list;

    match args.sub_command {
        SubCommands::SetIncome => set_income(),
        SubCommands::SetExpense => set_expense(),
        SubCommands::GetBalance => get_balance(&is_listed),
    }
}
