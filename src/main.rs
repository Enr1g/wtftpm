use clap::{value_parser, CommandFactory, Parser};
use clap_complete::Shell;
use wtftpm::config::{Config, SecretValue};

#[derive(Parser, Debug)]
#[command(rename_all = "lower")]
enum Cli {
    #[command(about = "Print example of config using libsecret")]
    LibsecretExample,
    #[command(about = "Print example of plaintext config")]
    PlainExample,
    #[command(about = "Print config path")]
    ConfigPath,
    #[command(about = "Check current config (warning: will print your plain secrets)")]
    CheckConfig,
    #[command(about = "Generate completion for shell")]
    Completion(CompletionArgs),
    #[command(about = "Store pin using libsecret")]
    StorePinWithLibsecret(StorePinWithLibsecretArgs),
}

#[derive(clap::Args, Debug)]
struct StorePinWithLibsecretArgs {
    #[arg(short, long, help="Namespace to store your pin. Use default if you have only one pkcs11 lib to wrap", default_value_t = SecretValue::default_libsecret_namespace())]
    namespace: String,
    #[arg(short, long, help = "Slot label for pin")]
    slot_label: String,
}

#[derive(clap::Args, Debug)]
struct CompletionArgs {
    #[arg(short, long, help="Desired shell", value_parser=value_parser!(Shell))]
    shell: Shell,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli {
        Cli::LibsecretExample => {
            let config_ls = Config::example_libsecret();
            let yaml = serde_yaml::to_string(&config_ls)?;
            println!("{}", yaml);
        }
        Cli::PlainExample => {
            let config_plain = Config::example_plain();
            let yaml = serde_yaml::to_string(&config_plain)?;
            println!("{}", yaml);
        }
        Cli::ConfigPath => {
            let config_path = Config::get_default_config_path()?;

            println!("{}", config_path.display());
        }
        Cli::CheckConfig => {
            let lw = wtftpm::overrides::get_lw().read().unwrap();
            let yaml = serde_yaml::to_string(lw.get_config())?;
            println!("{}", yaml);
        }
        Cli::Completion(args) => clap_complete::generate(
            args.shell,
            &mut Cli::command_for_update(),
            std::env::current_exe()?.display().to_string(),
            &mut std::io::stdout(),
        ),
        Cli::StorePinWithLibsecret(args) => {
            let StorePinWithLibsecretArgs {
                namespace,
                slot_label,
            } = args;

            let pin = rpassword::prompt_password(format!(
                "[namespace = {}][slot_label = {}] enter pin:",
                namespace, slot_label
            ))?;

            SecretValue::save_into_libsecret(&namespace, &slot_label, &pin)?;
        }
    }

    Ok(())
}
