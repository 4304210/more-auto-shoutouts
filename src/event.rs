use std::thread;

use log::error;
use mhw_toolkit::game_util::WeaponType;

use crate::{
    game_context::{Context, Fsm},
    triggers::TriggerManager,
};

/// 事件
#[derive(Debug)]
pub enum Event {
    LoadTriggers {
        trigger_mgr: TriggerManager,
    },
    LongswordLevelChanged {
        new: i32,
        old: i32,
        ctx: Context,
    },
    WeaponTypeChanged {
        new: WeaponType,
        old: WeaponType,
        ctx: Context,
    },
    QuestStateChanged {
        new: i32,
        old: i32,
        ctx: Context,
    },
    FsmChanged {
        new: Fsm,
        old: Fsm,
        ctx: Context,
    },
    UseItem {
        item_id: i32,
        ctx: Context,
    },
    InsectGlaive {
        ctx: Context,
    },
    ChargeBlade {
        ctx: Context,
    },
}

impl Event {
    pub fn extract_ctx(&self) -> Context {
        match self {
            Event::LoadTriggers { .. } => {
                error!("trying to get context from Event::LoadTriggers, panicked");
                // 只是防止没打出日志退出，可能大概有用吧
                thread::sleep(std::time::Duration::from_millis(500));
                panic!("trying to get context from Event::LoadTriggers, panicked")
            }
            Event::LongswordLevelChanged { ctx, .. } => ctx.clone(),
            Event::WeaponTypeChanged { ctx, .. } => ctx.clone(),
            Event::QuestStateChanged { ctx, .. } => ctx.clone(),
            Event::FsmChanged { ctx, .. } => ctx.clone(),
            Event::InsectGlaive { ctx } => ctx.clone(),
            Event::ChargeBlade { ctx } => ctx.clone(),
            Event::UseItem { ctx, .. } => ctx.clone(),
        }
    }
}
