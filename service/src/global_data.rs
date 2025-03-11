use actix::prelude::*;
use tracing::info;
use utils::err::ResultErr;

#[derive(Message)]
#[rtype(result = "Result<usize, ResultErr>")]
pub struct MsgData {
    pub count: usize,
}

pub struct GlobalData {
    count: usize,
}

impl GlobalData {
    /// 新建GlobalData
    pub fn new(count: usize) -> GlobalData {
        GlobalData { count }
    }
    
    /// 获取count
    pub fn count(&self) -> usize {
        self.count
    }
    
    
}

impl Actor for GlobalData {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Global data started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Global data stopped");
    }
}

impl Handler<MsgData> for GlobalData {
    type Result = Result<usize, ResultErr>;

    fn handle(&mut self, msg: MsgData, _ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.count;
        Ok(self.count)
    }
}
