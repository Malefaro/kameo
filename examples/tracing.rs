use kameo::prelude::*;

#[derive(Actor)]
struct First {
    counter: usize,
    middles: Vec<ActorRef<Middle>>,
}

struct FirstMsg;
impl Message<FirstMsg> for First {
    type Reply = String;

    async fn handle(&mut self, msg: FirstMsg, ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        self.counter += 1;
        let middle = self.middles.get(self.counter % self.middles.len()).unwrap();
        // ctx.forward(middle, ) TODO: how to be with forwarded messages?
        let r = middle.ask(MiddleMsg(self.counter)).await;
        todo!()
    }
}

struct MiddleMsg(usize);
#[derive(Actor)]
struct Middle;

impl Message<MiddleMsg> for Middle {
    type Reply = String;

    async fn handle(
        &mut self,
        msg: MiddleMsg,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        todo!()
    }
}

#[derive(Actor)]
struct Last;

#[tokio::main]
async fn main() {}
