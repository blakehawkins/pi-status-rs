extern crate slack;
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
        let _ = cli.send_message("#random", "Hello world! (rtm)");
        // post a message as a user to the web api
        let _ = cli.post_message("#random", "hello world! (postMessage)", None);
        // set a channel topic via the web api
        // let _ = cli.set_topic("#random", "bots rule!");
    }
}

fn main() {
    let api_key = "xoxp-99574921762-99564265797-146724218293-6e27b1a8e8dad22840a8fc83bfd2903e";
    let mut handler = SlackHandler;
    let mut client = slack::RtmClient::new(&api_key);
    let _ = client.login_and_run::<SlackHandler>(&mut handler).unwrap();
    println!("{}", client.get_name().unwrap());
    println!("{}", client.get_team().unwrap().name);
}
