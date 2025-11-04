use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version, author, about)]
#[command(arg_required_else_help = true)]
pub struct CLIArgs {
    /// Initialize configuration file (In Linux: $XDG_CONFIG_HOME/kakei)
    #[arg(long, default_value_t = false)]
    pub initialize_configuration_file: bool,

    /// Initialize default kakeibo file (In Linux: $XDG_DATA_HOME/kakei)
    #[arg(long, default_value_t = false)]
    pub initialize_default_kakeibo: bool,
}
