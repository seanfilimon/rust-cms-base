use std::time::Instant;

use actix::prelude::*;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::errors::MyError;

pub struct MyWs {
    hb: Instant,
}

impl MyWs {
    fn new() -> Self { Self { hb: Instant::now() } }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(std::time::Duration::from_secs(5), |act, ctx| {
            if Instant::now().duration_since(act.hb) > std::time::Duration::from_secs(10) {
                println!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("WS: {msg:?}");
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

pub async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, MyError> {
    let resp = ws::start(MyWs::new(), &r, stream)?;
    println!("WS: Handshake response: {:?}", resp);
    Ok(resp)
}
