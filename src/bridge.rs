use std::process::{Command as ProcessCommand, exit};
use std::fs;
use std::env;
use std::path::Path;
use regex::Regex;
use crate::constants;

pub fn check_container_exists(name: &str) -> bool {
    let output = ProcessCommand::new(constants::DOCKER_CMD)
        .args(["ps", "-a", "--format", "{{.Names}}"])
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let regex = Regex::new(&format!("^{}$", regex::escape(name))).unwrap();
            stdout.lines().any(|line| regex.is_match(line.trim()))
        }
        Err(_) => false,
    }
}

pub fn check_container_running(name: &str) -> bool {
    let output = ProcessCommand::new(constants::DOCKER_CMD)
        .args(["ps", "--format", "{{.Names}}"])
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let regex = Regex::new(&format!("^{}$", regex::escape(name))).unwrap();
            stdout.lines().any(|line| regex.is_match(line.trim()))
        }
        Err(_) => false,
    }
}

pub fn start_container(name: &str) {
    let status = ProcessCommand::new(constants::DOCKER_CMD)
        .args(["start", name])
        .status()
        .expect("Failed to start container");

    if !status.success() {
        eprintln!("Failed to start container {}", name);
        exit(1);
    }
}

fn create_docker_compose(container_name: &str, volume_name: &str) -> String {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let current_dir_str = current_dir.to_string_lossy();

    // Get resource limits from environment variables or use defaults
    let cpus = env::var("SB_CPUS").unwrap_or_else(|_| "4".to_string());
    let memory = env::var("SB_MEMORY").unwrap_or_else(|_| "8g".to_string());

    let sandbox_dir = Path::new(".sandbox");
    fs::create_dir_all(sandbox_dir).expect("Failed to create .sandbox directory");

    // Load template from external file
    let template = include_str!("../templates/docker-compose.yml");
    let compose_content = template
        .replace("{{CONTAINER_NAME}}", container_name)
        .replace("{{CURRENT_DIR}}", &current_dir_str)
        .replace("{{VOLUME_NAME}}", volume_name)
        .replace("{{CPUS}}", &cpus)
        .replace("{{MEMORY}}", &memory);

    let compose_file = sandbox_dir.join(format!(".docker-compose-{}.yml", container_name));
    fs::write(&compose_file, compose_content).expect("Failed to write docker-compose file");
    compose_file.to_string_lossy().to_string()
}

pub fn create_container_with_compose(name: &str) {
    // Check if container already exists - if so, don't recreate it
    if check_container_exists(name) {
        return;
    }

    let volume_name = "sandbox-home".to_string(); // Shared home volume across all sandboxes
    let compose_file = create_docker_compose(name, &volume_name);

    let status = ProcessCommand::new(constants::DOCKER_COMPOSE_CMD)
        .args(["-f", &compose_file, "up", "-d"])
        .status()
        .expect("Failed to create container with docker-compose");

    if !status.success() {
        eprintln!("Failed to create container {} with docker-compose", name);
        exit(1);
    }
}

pub fn execute_in_container(name: &str, args: &[String]) {
    let mut cmd = ProcessCommand::new(constants::DOCKER_CMD);
    cmd.args(["exec", "-it", name, constants::SANDBOX_SCRIPT, constants::CLAUDE_COMMAND]);

    // Add all claude arguments
    for arg in args {
        cmd.arg(arg);
    }

    let status = cmd.status().expect("Failed to execute command in container");

    if let Some(code) = status.code() {
        exit(code);
    } else {
        exit(1);
    }
}

pub fn execute_bash_in_container(name: &str) {
    let status = ProcessCommand::new(constants::DOCKER_CMD)
        .args(["exec", "-it", name, constants::SANDBOX_SCRIPT])
        .status()
        .expect("Failed to execute bash in container");

    if let Some(code) = status.code() {
        exit(code);
    } else {
        exit(1);
    }
}

fn stop_and_remove_container(container_name: &str) -> bool {
    if !check_container_exists(container_name) {
        println!("No sandbox container found");
        return true;
    }

    println!("Stopping sandbox container...");
    let _ = ProcessCommand::new(constants::DOCKER_CMD)
        .args(["stop", container_name])
        .status();

    println!("Removing sandbox container...");
    let status = ProcessCommand::new(constants::DOCKER_CMD)
        .args(["rm", container_name])
        .status()
        .expect("Failed to remove container");

    if status.success() {
        println!("✅ Container removed");
        true
    } else {
        eprintln!("⚠️  Failed to remove container");
        false
    }
}

fn remove_docker_image(image_name: &str) -> bool {
    println!("Removing sandbox Docker image...");
    let status = ProcessCommand::new(constants::DOCKER_CMD)
        .args(["rmi", image_name])
        .status()
        .expect("Failed to remove image");

    if status.success() {
        println!("✅ Docker image removed");
        true
    } else {
        println!("⚠️  No sandbox image found or failed to remove");
        false
    }
}

fn remove_persistent_volume(volume_name: &str) -> bool {
    println!("Removing persistent volume...");
    let status = ProcessCommand::new(constants::DOCKER_CMD)
        .args(["volume", "rm", volume_name])
        .status()
        .expect("Failed to remove volume");

    if status.success() {
        println!("✅ Persistent volume removed");
        true
    } else {
        println!("⚠️  No persistent volume found or failed to remove");
        false
    }
}

pub fn clean_sandbox(hard_clean: bool) {
    println!("🧹 Cleaning sandbox artifacts...");

    stop_and_remove_container(constants::CONTAINER_NAME);
    remove_docker_image(constants::IMAGE_NAME);

    if hard_clean {
        remove_persistent_volume(constants::VOLUME_NAME);
    }

    println!("✅ Sandbox cleanup completed!");
}