// register_macro! {
//     /** 说你好 */
//     SayHello,
//     /** 说话 */
//     Say,
// }

// use actix::Message;

// use crate::cmds::Cmd;

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct SayHello {
//     pub from: &'static str,
//     pub data: String,
// }

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Say {
//     pub from: &'static str,
//     pub data: String,
// }


// impl Cmd for SayHello {
//     fn name(&self) -> &'static str {
//         "SayHello"
//     }
//     fn route(addr: actix::Addr<crate::ws_conn::WsConn>, data: String) {
//         addr.do_send(Self { from: "SayHello", data })
//     }
// }
// impl Cmd for Say {
//     fn name(&self) -> &'static str {
//         "Say"
//     }
//     fn route(addr: actix::Addr<crate::ws_conn::WsConn>, data: String) {
//         addr.do_send(Self { from: "Say", data })
//     }
// }


use std::collections::HashMap;
use actix::{Addr, Message};
use crate::cmd::Cmd;
use crate::ws_conn::WsConn;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Chat {
    pub cmd: &'static str,
    pub text: String,
}

impl Chat {
    fn get_cmd() -> &'static str { "chat" }
    pub fn new(text: String) -> Self {
        Chat { cmd: Self::get_cmd(), text }
    }
}

impl Cmd for Chat {
    fn name(&self) -> &'static str { stringify!(Chat) }

    fn route(addr: Addr<WsConn>, data: String) {
        addr.do_send(Chat::new(data))
    }
}

lazy_static::lazy_static! {
    pub static ref CMD_MAP: HashMap<&'static str, fn(addr: Addr<WsConn>, data: String)> = {
        let mut map = HashMap::new();
        map.insert(Chat::get_cmd(), Chat::route as fn(addr: Addr<WsConn>, data: String));
        map
    };
}