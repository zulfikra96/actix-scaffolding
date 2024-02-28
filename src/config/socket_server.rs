use std::collections::{HashMap, HashSet};

use actix::{Actor, Context, Handler, MessageResult, Recipient};
// use actix_web_actors::ws::Message;
use serde::Deserialize;

use crate::config::messages::{self, *};
pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

#[derive(Debug, Clone)]
pub struct ChatServer {
    pub sessions: HashMap<uuid::Uuid, Recipient<messages::Message>>,
    pub rooms: HashMap<String, HashSet<uuid::Uuid>>,
}

impl Default for ChatServer {
    fn default() -> Self {
        ChatServer {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

impl ChatServer {
    pub fn send_message(&self, to_id: uuid::Uuid, messages: &str) {
        if let Some(addr) = self.sessions.get(&to_id) {
            addr.do_send(messages::Message(messages.to_owned()))
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        println!("Someone Disconnected");

        let mut rooms: Vec<String> = Vec::new();

        if self.sessions.remove(&msg.id).is_some() {
            for (name, session) in &mut self.rooms {
                if session.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }
    }
}

impl Handler<Connect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        println!("Someone joined");
        self.rooms
            .entry(msg.room_id.clone())
            .or_insert_with(HashSet::new)
            .insert(msg.self_id);

        self.rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
            .for_each(|_conn_id| {
                ()
                // self.send_message(*conn_id, &format!("{} just joined!", msg.self_id))
            });
        self.sessions.insert(msg.self_id, msg.addr);
    }
}

impl Handler<ListRooms> for ChatServer {
    type Result = MessageResult<ListRooms>;
    fn handle(&mut self, _: ListRooms, _: &mut Self::Context) -> Self::Result {
        let mut rooms: Vec<String> = Vec::new();

        for key in self.rooms.keys() {
            rooms.push(key.to_owned());
        }

        MessageResult(rooms)
    }
}

#[derive(Debug, Deserialize, serde::Serialize)]
struct MessageJson {
    token: String,
    message: String,
    created_at: i64,
    r#type: String,
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) -> Self::Result {
        let _room = msg.room.clone();
        let _id = msg.id.clone();
        self.rooms
            .get(&msg.room)
            .unwrap()
            .iter()
            .for_each(|client| {
                self.send_message(*client, msg.msg.as_str());
            })
    }
}
