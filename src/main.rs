use clap::{Arg, ArgMatches, Command};
use std::io::{self, Write};

mod bridge;
mod constants;
mod utils;


fn main() {
    let app = Command::new("sb")
        .about("Sandbox CLI tool dispatcher")
        .version("0.1.0")
        .subcommand_required(false)
        .subcommand(
            Command::new("claude")
                .about("Launch Claude Code environment")
                .allow_external_subcommands(true)
                .disable_help_flag(true)
                .disable_version_flag(true)
                .ignore_errors(true)
                .arg(
                    Arg::new("args")
                        .help("Arguments to pass to claude command")
                        .num_args(0..)
                        .trailing_var_arg(true)
                        .allow_hyphen_values(true)
                )
        )
        .subcommand(
            Command::new("clean")
                .about("Remove sandbox container and artifacts")
                .arg(
                    Arg::new("hard")
                        .long("hard")
                        .help("Also remove persistent volume (with confirmation)")
                        .action(clap::ArgAction::SetTrue)
                )
        );

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("claude", sub_matches)) => handle_claude(sub_matches),
        Some(("clean", sub_matches)) => handle_clean(sub_matches),
        None => handle_default_shell(),
        _ => unreachable!(),
    }
}

fn handle_clean(matches: &ArgMatches) {
    let hard_clean = matches.get_flag("hard");

    if hard_clean {
        println!("⚠️  WARNING: --hard will remove ALL sandbox artifacts including persistent data!");
        print!("Are you sure you want to continue? (y/N): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        if !input.trim().to_lowercase().starts_with('y') {
            println!("Cancelled.");
            return;
        }
    }

    bridge::clean_sandbox(hard_clean);
}

fn handle_default_shell() {
    let container_name = utils::generate_container_name();

    // Check if sandbox container exists
    let container_exists = bridge::check_container_exists(&container_name);

    if container_exists {
        // Check if container is running
        let container_running = bridge::check_container_running(&container_name);

        if container_running {
            println!("Connecting to running sandbox container for bash session...");
            bridge::execute_bash_in_container(&container_name);
        } else {
            println!("Starting existing sandbox container for bash session...");
            bridge::start_container(&container_name);
            bridge::execute_bash_in_container(&container_name);
        }
    } else {
        println!("Creating new sandbox container for bash session...");
        bridge::create_container_with_compose(&container_name);
        bridge::execute_bash_in_container(&container_name);
    }
}

fn handle_claude(matches: &ArgMatches) {
    // Collect all arguments to pass to claude
    let mut args = Vec::new();
    if let Some(claude_args) = matches.get_many::<String>("args") {
        for arg in claude_args {
            args.push(arg.as_str());
        }
    }

    // Validate Claude arguments for security
    let validated_args = match utils::validate_claude_args(&args) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Error: Invalid argument - {}", err);
            std::process::exit(1);
        }
    };

    let container_name = utils::generate_container_name();

    // Check if sandbox container exists
    let container_exists = bridge::check_container_exists(&container_name);

    if container_exists {
        // Check if container is running
        let container_running = bridge::check_container_running(&container_name);

        if container_running {
            println!("Connecting to running sandbox container for Claude Code...");
            bridge::execute_in_container(&container_name, &validated_args);
        } else {
            println!("Starting existing sandbox container for Claude Code...");
            bridge::start_container(&container_name);
            bridge::execute_in_container(&container_name, &validated_args);
        }
    } else {
        println!("Creating new sandbox container for Claude Code...");
        bridge::create_container_with_compose(&container_name);
        bridge::execute_in_container(&container_name, &validated_args);
    }
}

