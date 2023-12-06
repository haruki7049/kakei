use clap::Parser;

fn main() {
    let args: Args = Args::parse();
    let is_listed: bool = args.list;

    match args.sub_command.as_str() {
        "set-income" => {
            println!("set-income is running...")
        }
        "get-balance" => {
            println!("{}", is_listed);
            println!("get-balance is running...")
        }
        "set-expense" => {
            println!("set-expense is running...");
        }
        _ => {
            eprintln!("Unknown command: {}", &args.sub_command);
        }
    }

}

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(help = "What you want to do")]
    pub sub_command: String,

    #[arg(long, help = "If you want kakei to show as list, use --list")]
    pub list: bool,
}
