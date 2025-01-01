use super::{Map, Player, Position, RunState, State};
use crate::{CombatStats, Viewshed};
use rltk::{console, Point, Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();
    let combat_stats = ecs.read_storage::<CombatStats>();

    for (_player, pos, viewshed) in (&players, &mut positions, &mut viewsheds).join() {
        let dest_idx = map.xy_idx(pos.x + dx, pos.y + dy);

        for potential_targets in map.tile_content[dest_idx].iter() {
            let target = combat_stats.get(*potential_targets);
            match target {
                None => {}
                Some(t) => {
                    console::log(&format!("Stab!"));
                    return;
                }
            }
        }
        if !map.blocked[dest_idx] {
            pos.x = min(79, max(0, pos.x + dx));
            pos.y = min(49, max(0, pos.y + dy));
            viewshed.dirty = true;
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => return RunState::Paused,
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                try_move_player(-1, 0, &mut gs.ecs)
            }

            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                try_move_player(1, 0, &mut gs.ecs)
            }

            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                try_move_player(0, -1, &mut gs.ecs)
            }

            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                try_move_player(0, 1, &mut gs.ecs)
            }
            VirtualKeyCode::Numpad9 | VirtualKeyCode::Y => try_move_player(1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad7 | VirtualKeyCode::U => try_move_player(-1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => try_move_player(1, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),
            _ => return RunState::Paused,
        },
    }
    RunState::Running
}
