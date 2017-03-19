extern crate slack;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
struct Config {
    slack_key: String,
}

struct SlackHandler;

#[allow(unused_variables)]
impl slack::EventHandler for SlackHandler {
    fn on_event(&mut self, cli: &mut slack::RtmClient, event: Result<slack::Event, slack::Error>, raw_json: &str) {
        println!("on_event(event: {:?}, raw_json: {:?})", event, raw_json);
    }

    fn on_ping(&mut self, cli: &mut slack::RtmClient) {
        println!("on_ping");
    }

    fn on_close(&mut self, cli: &mut slack::RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &mut slack::RtmClient) {
        println!("on_connect");
        // Do a few things using the api:
        // send a message over the real time api websocket
        let _ = cli.send_message("#pi-status", ".");
        // post a message as a user to the web api
        // let _ = cli.post_message("#pi-status", "hello world! (postMessage)", None);
        // set a channel topic via the web api
        // let _ = cli.set_topic("#pi-status", "bots rule!");
    }
}

fn main() {
    // Get the slack API key.
    let mut f = match File::open("config.toml") {
        Ok(val) => val,
        _ => panic!("Couldn't find config.toml!"),
    };
    let mut s = String::new();
    let _ = f.read_to_string(&mut s);
    let conf: Config = match toml::from_str(&s) {
        Ok(val) => val,
        _ => panic!("Couldn't deserialise config.toml!"),
    };
    let api_key = conf.slack_key;

    let mut handler = SlackHandler;
    let mut client = slack::RtmClient::new(&api_key);
    let _ = client.login_and_run::<SlackHandler>(&mut handler).unwrap();
    println!("{}", client.get_name().unwrap());
    println!("{}", client.get_team().unwrap().name);
}
