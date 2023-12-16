use kakei::commands::{
    set_income,
    set_expense,
    get_balance,
    SubCommands,
    Args,
};
use clap::Parser;

fn main() {
    let args: Args = Args::parse();
    let is_listed: bool = args.list;

    match args.sub_command {
        SubCommands::SetIncome => set_income(),
        SubCommands::SetExpense => set_expense(),
        SubCommands::GetBalance => get_balance(&is_listed),
    }
}
