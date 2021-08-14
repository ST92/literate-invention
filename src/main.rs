use std::pin::Pin;

use actix::prelude::*;

mod document;
mod dataobjects;
mod data;


struct Ping;

impl Message for Ping {
    type Result = Result<bool, std::io::Error>;
}

#[derive(Default)]
struct DistantReceiver {
    bitswitch: bool
}

impl Actor for DistantReceiver {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {}

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        Running::Stop
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {}

}

impl Handler<Ping> for DistantReceiver {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
        self.bitswitch = !self.bitswitch;

        Result::Ok(self.bitswitch)
    }
}

#[actix::main]
async fn main() {
    let address = DistantReceiver::start_default();

    let result = address.send(Ping).await;

    if let Ok(Ok(ping)) = result {
        println!("Receiver responded: {:}", ping);
    }

    let result = address.send(Ping).await;

    if let Ok(Ok(ping)) = result {
        println!("Receiver responded: {:}", ping);
    }

    let result = address.send(Ping).await;

    if let Ok(Ok(ping)) = result {
        println!("Receiver responded: {:}", ping);
    }
}
