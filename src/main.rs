use std::env;

use serenity::async_trait;
use serenity::model::prelude::Embed;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[group]
#[commands(status)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    //let token = env::var("DISCORD_TOKEN").expect("token");
    let token: String = "".to_string();
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn status(ctx: &Context, msg: &Message) -> CommandResult {
    let address: Vec<&str> = &msg.content.split(" ");
    let url: String = format!("https://api.mcsrvstat.us/2/{}", address[1]);

    //let mut res: status::Status = status::Status::new();

    let status = tokio::task::spawn_blocking(move || {
        return status_grab(&url);
    }).await.unwrap().unwrap();

    msg.reply(ctx, status.hostname).await?;

    Ok(())
}


//------------

use reqwest::Error;
use reqwest::blocking;

pub mod status {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Debug {
        pub ping: bool,
        pub query: bool,
        pub srv: bool,
        pub querymismatch:bool,
        pub ipinsrv: bool,
        pub cnameinsrv: bool,
        pub animatedmotd: bool,
        pub cachetime: i32,
        pub apiversion: i32,
        pub error: Option<Error>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Error {
        ping: String,
        query: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Motd {
        pub raw: Vec<String>,
        pub clean:Vec<String>,
        pub html: Vec<String>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Players {
        pub online: i32,
        pub max :i32,
        pub list: Option<Vec<String>>,
        pub uuid: Option<Vec<String>>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Plugins {
        pub names: Vec<String>,
        pub raw: Vec<String>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Mods {
        pub names: Vec<String>,
        pub raw: Vec<String>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Info {
        pub raw: Vec<String>,
        pub clean:Vec<String>,
        pub html: Vec<String>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Status {
        pub ip: String,
        pub port: i32,
        pub debug: Debug,
        pub motd: Motd,
        pub players: Players,
        pub version: String,
        pub online: bool,
        pub protocol: i32,
        pub hostname: String,
        pub icon: String,
        pub software: String,
        pub map: String,
        pub plugins: Option<Plugins>,
        pub mods: Option<Mods>,
        pub info: Option<Info>,
        pub gamemode: Option<String>, 
        pub serverid: Option<String>,
    }

    impl Status {
        pub fn new() -> Self {
            Self {
                ip: "127.0.0.1".to_string(),
                port: 25565,
                debug: Debug {
                    ping:false,
                    query:false,
                    srv:false,
                    querymismatch:false,
                    ipinsrv:false,
                    cnameinsrv:false,
                    animatedmotd:false,
                    cachetime:0,
                    apiversion:2,
                    error: Some (Error {
                        ping:"No address to query".to_string(),
                        query:"Could not create socket: php_network_getaddresses: getaddrinfo for  failed: Name or service not known".to_string()
                    })
                },
                motd: Motd {
                    raw: ["null".to_string()].to_vec(),
                    clean: ["null".to_string()].to_vec(),
                    html: ["null".to_string()].to_vec(),
                },
                players: Players {
                    online: 0,
                    max : 0,
                    list: Some(["null".to_string()].to_vec()),
                    uuid: Some(["null".to_string()].to_vec()),
                },
                version: "0".to_string(),
                online: false,
                protocol: 0,
                hostname: "null".to_string(),
                icon: "null".to_string(),
                software: "null".to_string(),
                map: "null".to_string(),
                plugins: Some(Plugins {
                    raw: ["null".to_string()].to_vec(),
                    names: ["null".to_string()].to_vec(),
                }),
                mods: Some(Mods {
                    raw: ["null".to_string()].to_vec(),
                    names: ["null".to_string()].to_vec(),
                }),
                info: Some(Info {
                    raw: ["null".to_string()].to_vec(),
                    clean: ["null".to_string()].to_vec(),
                    html: ["null".to_string()].to_vec(),
                }),
                gamemode: Some("null".to_string()),
                serverid: Some("null".to_string()),
            }
        }
    }
}

fn status_grab(url: &String) -> Result<status::Status, Error> {
    let result = reqwest::blocking::get(url)?;
    let res: status::Status = result.json()?;
    return Ok(res);
}