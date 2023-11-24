use crate::mqtt::{DRIVERS, DS18B20, MAX3010X, MPU6050};
use actix::{prelude::*, Actor, AsyncContext, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use anyhow::Result;
use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    sync::{atomic::AtomicUsize, Arc},
};
use tokio::{sync::broadcast::Sender, task};
use uuid::Uuid;

lazy_static! {
    static ref DRIVERS_SET: HashSet<String> = {
        let mut set = HashSet::new();
        set.insert(DS18B20.to_string());
        set.insert(MAX3010X.to_string());
        set.insert(MPU6050.to_string());
        set
    };
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(String)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {}
    }
}

impl Actor for Server {
    type Context = actix::Context<Self>;
}

impl Handler<Connect> for Server {
    type Result = String;

    fn handle(&mut self, _: Connect, _: &mut actix::Context<Self>) -> Self::Result {
        Uuid::new_v4().to_string()
    }
}

struct Session {
    pub id: String,
    pub addr: actix::Addr<Server>,
    pub txs: Arc<HashMap<String, Sender<String>>>,
}

impl Handler<Message> for Session {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();

        self.addr
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                actix::fut::ready(())
            })
            .wait(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let message = match msg {
            Ok(ws::Message::Ping(text)) => Some(String::from_utf8_lossy(&text).to_string()),
            Ok(ws::Message::Text(text)) => {
                Some(String::from_utf8_lossy(&text.as_bytes()).to_string())
            }
            Ok(ws::Message::Binary(bin)) => Some(String::from_utf8_lossy(&bin).to_string()),
            _ => None,
        };

        if let Some(message) = message {
            if let Some(driver) = self.txs.get(&message) {
                let mut rx = driver.subscribe();
                let addr = ctx.address();

                task::spawn(async move {
                    loop {
                        match rx.recv().await {
                            Ok(values) => {
                                addr.do_send(Message(values));
                            }
                            Err(e) => println!("error: {:?}", e),
                        }
                    }
                });
            }
        }
    }
}

pub async fn route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<actix::Addr<Server>>,
    tx: web::Data<Arc<HashMap<String, Sender<String>>>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        Session {
            id: String::new(),
            addr: srv.get_ref().clone(),
            txs: tx.get_ref().clone(),
        },
        &req,
        stream,
    )
}
