use actix::Handler;

use crate::{router::Chat, ws_conn::WsConn};

// impl Handler<SayHello> for WsConn {
//     type Result = ();
//
//     fn handle(&mut self, _: SayHello, ctx: &mut Self::Context) -> Self::Result {
//         ctx.text("hello!");
//     }
// }

impl Handler<Chat> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: Chat, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.text);
    }
}