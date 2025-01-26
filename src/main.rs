use clap::{Arg, Command};
use reqwest::blocking::Client;
use serde::Deserialize;

const DOCKER_SOCKET: &str = "http://localhost/v1.41";

#[derive(Deserialize)]
struct Container {
    Id: String,
    Names: Vec<String>,
    State: String,
    Status: String,
}

fn main() {
    let matches = Command::new("dockerctl")
        .version("1.0")
        .author("Your Name <youremail@example.com>")
        .about("Command-line tool to control Docker containers")
        .subcommand(
            Command::new("list")
                .about("Lists all containers")
        )
        .subcommand(
            Command::new("start")
                .about("Starts a container")
                .arg(Arg::new("id").required(true).help("Container ID")),
        )
        .subcommand(
            Command::new("stop")
                .about("Stops a container")
                .arg(Arg::new("id").required(true).help("Container ID")),
        )
        .get_matches();

    let client = Client::new();

    if let Some(_) = matches.subcommand_matches("list") {
        list_containers(&client);
    } else if let Some(sub_m) = matches.subcommand_matches("start") {
        let id = sub_m.get_one::<String>("id").unwrap();
        start_container(&client, id);
    } else if let Some(sub_m) = matches.subcommand_matches("stop") {
        let id = sub_m.get_one::<String>("id").unwrap();
        stop_container(&client, id);
    }
}

fn list_containers(client: &Client) {
    let url = format!("{}/containers/json?all=true", DOCKER_SOCKET);
    let response = client.get(&url).send().unwrap().text().unwrap();

    let containers: Vec<Container> = serde_json::from_str(&response).unwrap();

    for container in containers {
        println!(
            "ID: {}, Name: {}, State: {}, Status: {}",
            container.Id,
            container.Names.join(", "),
            container.State,
            container.Status
        );
    }
}

fn start_container(client: &Client, id: &str) {
    let url = format!("{}/containers/{}/start", DOCKER_SOCKET, id);
    client.post(&url).send().unwrap();
    println!("Started container: {}", id);
}

fn stop_container(client: &Client, id: &str) {
    let url = format!("{}/containers/{}/stop", DOCKER_SOCKET, id);
    client.post(&url).send().unwrap();
    println!("Stopped container: {}", id);
}
