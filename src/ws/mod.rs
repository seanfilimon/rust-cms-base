use std::time::Instant;

use crate::errors::MyError;
use actix::prelude::*;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

struct MyWs {
    hb: Instant,
}

impl MyWs {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }

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

#[derive(Message, Clone)]
#[rtype(result = "()")]
struct ServerEvent {
    pub event: String,
}

#[derive(Message)]
#[rtype(result = "()")]

struct RegisterWsClient {
    client: Addr<MyWs>,
}

#[derive(Default, Clone)]
pub struct ServerMonitor {
    clients: Vec<Addr<MyWs>>,
}

impl ServerMonitor {
    pub fn new() -> Addr<Self> {
        ServerMonitor::default().start()
    }
}

impl Actor for ServerMonitor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(std::time::Duration::from_secs(5), |act, _| {
            for client in act.clients.iter() {
                client.do_send(ServerEvent {
                    event: "alive".to_string(),
                });
            }
        });
    }
}

impl Handler<RegisterWsClient> for ServerMonitor {
    type Result = ();

    fn handle(&mut self, msg: RegisterWsClient, _: &mut Context<Self>) {
        self.clients.push(msg.client);
    }
}

impl Handler<ServerEvent> for MyWs {
    type Result = ();

    fn handle(&mut self, msg: ServerEvent, ctx: &mut Self::Context) {
        ctx.text(msg.event);
    }
}

pub async fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    data: web::Data<Addr<ServerMonitor>>,
) -> Result<HttpResponse, MyError> {
    let (addr, res) = ws::WsResponseBuilder::new(MyWs::new(), &r, stream).start_with_addr()?;
    data.get_ref().do_send(RegisterWsClient { client: addr });
    println!("WS: Handshake response: {:?}", res);
    Ok(res)
}
