use super::*;
use crate::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::gameobjtype::UpdateData;
use crate::event::ActionUpdates;

use sdl2::event::{
    Event,
    WindowEvent,
};
use sdl2::EventPump;

use hecs::Entity as EntityId;

use std::cmp::Ordering;

fn handle_err(label: &str, res: std::result::Result<(), impl std::fmt::Display>) {
    if let Err(res) = res {
        eprintln!("error: {}: {}", label, res);
    }
}

impl<'a> Game<'a> {
    pub fn update(&mut self, event_pump: &mut EventPump) -> Result<bool> {
        let timer = debug::Timer::new("handling events");
        let mut updates = ActionUpdates::new();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return Ok(true);
                },
                Event::MouseWheel { y, .. } => {
                    self.tile_scale =
                        (self.tile_scale as i32).saturating_sub(y).min(70).max(30) as u32;
                },
                Event::KeyDown { scancode, .. } => {
                    if let Some(scancode) = scancode {
                        updates.register_event(&self.action_handler, scancode, true); }
                },
                Event::KeyUp { scancode, .. } => {
                    if let Some(scancode) = scancode {
                        updates.register_event(&self.action_handler, scancode, false); }
                },
                Event::MouseButtonDown { mouse_btn, .. } => {
                    updates.register_event(&self.action_handler, mouse_btn, true);
                },
                Event::MouseButtonUp { mouse_btn, .. } => {
                    updates.register_event(&self.action_handler, mouse_btn, false);
                },
                Event::Window { win_event, .. } => {
                    match win_event {
                        WindowEvent::Resized(width, height) => {
                            self.screen_size = (width, height);
                            updates.clear();
                            break
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        timer.done();

        if let Ok(mut player) = self.ecs.get::<&mut Player>(self.player) {
            player.action_state.update(&self.action_handler, &updates);
        }

        self.update_loaded(false)?;

        let ui_hovered = self.update_ui(event_pump);
        self.update_player(event_pump, ui_hovered)?;

        let timer = debug::Timer::new("getting update fns");
        let mut id_update_fn_pairs = Vec::new();
        for id in &self.loaded.ids {
            if let Ok(update_fn) = self.ecs.get::<&UpdateFn>(*id) {
                id_update_fn_pairs.push((
                    *id,
                    self.types.get_update_fn_from_id(update_fn.id)
                ));
            }
        }
        timer.done();

        let update_data = UpdateData{};

        let timer = debug::Timer::new("updating");
        for (id, update_fn) in id_update_fn_pairs {
            handle_err(&format!("updating entity {}", id.id()).to_string(),
                update_fn(&mut self.ecs, id, &update_data));
        }
        timer.done();

        Ok(false)
    }

    fn update_player(&mut self, event_pump: &mut EventPump, ui_hovered: bool) -> Result<()> {
        let actions = self.ecs.get::<&Player>(self.player)?.action_state.clone();

        let speed =
            if actions.key("run")
                { 0.2 } else { 0.1 };

        if let Ok(mut pos) = self.ecs.get::<&mut Position>(self.player) {
            if actions.key("move_right") { pos.move_x(speed); }
            if actions.key("move_left")  { pos.move_x(-speed); }
            if actions.key("move_down")  { pos.move_y(speed); }
            if actions.key("move_up")    { pos.move_y(-speed); }
        }

        if !ui_hovered {
            let selected =
                if actions.key("attack") {
                    if let Ok(player) = self.ecs.get::<&Player>(self.player) {
                        player.selected
                    } else { None }
                } else { None };

            if let Some(selected) = selected {
                handle_err("despawning selected entity",
                    self.ecs.despawn(selected).into());
            }

            if let Ok(mut player) = self.ecs.get::<&mut Player>(self.player) {
                player.selected = 'block: {
                    let mouse = event_pump.mouse_state();

                    for i in (0..self.loaded.ids.len()).rev() {
                        let id = self.loaded.ids[i];
                        if id == self.player { continue }
                        if let Ok(rect) = self.get_sdl_rect(id) {
                            if rect.contains_point((mouse.x(), mouse.y())) {
                                break 'block Some(id);
                            }
                        }
                    }

                    None
                };
            }
        }

        Ok(())
    }

    pub(super) fn update_loaded(&mut self, force: bool) -> Result<()> {
        if !force && !self.loaded_update_counter.count() {
            return Ok(())
        }

        let do_update = force ||
            self.ecs.get::<&Position>(self.player)?.clone().chunk() != self.player_chunk;

        if do_update {
            self.player_chunk = self.ecs.get::<&Position>(self.player)?.chunk();

            let chunks = {
                let player_chunk = self.ecs.get::<&Position>(self.player)?.chunk();
                core::array::from_fn::<ChunkPos, {RENDER_DISTANCE.pow(2)}, _>(|i|
                    ChunkPos::new(
                        player_chunk.x
                            + (i % RENDER_DISTANCE) as ChunkPosType
                            - RENDER_DISTANCE as ChunkPosType / 2,
                        player_chunk.y
                            + (i / RENDER_DISTANCE) as ChunkPosType
                            - RENDER_DISTANCE as ChunkPosType / 2,
                )).to_vec()
            };

            let timer = debug::Timer::new("generating chunks?");
            for chunk in &chunks {
                if !self.chunks.contains(chunk) {
                    self.generate_chunk(chunk.clone())?;
                }
            }
            timer.done();
            
            let timer = debug::Timer::new("getting ids");

            let mut render_order: [Vec<EntityId>; 5] =
                core::array::from_fn(|_| Vec::new());

            for (id, (pos,)) in self.ecs.query::<(&Position,)>().iter() {
                if chunks.contains(&pos.chunk()) {
                    render_order[
                        if id == self.player {
                            2
                        } else {
                            pos.order()
                        }
                    ].push(id);
                }
            }

            render_order[3].sort_unstable_by(|id1, id2| {
                let pos1 = self.ecs.get::<&Position>(*id1).unwrap();
                let pos2 = self.ecs.get::<&Position>(*id2).unwrap();

                let pos1 = pos1.x() + pos1.y();
                let pos2 = pos2.x() + pos2.y();

                if pos1 < pos2 {
                    Ordering::Less 
                } else {
                    Ordering::Greater
                }
            });

            let ids = render_order.iter().flat_map(|v| v.iter()).cloned().collect();

            timer.done();

            self.loaded = Loaded{
                ids,
            };
        }

        Ok(())
    }

    fn update_ui(&mut self, event_pump: &mut EventPump) -> bool {
        use crate::ui::*;

        // let mouse = event_pump.mouse_state();
        // let mx = mouse.x();
        // let my = mouse.y();
        // let mouse_delta = (mx - self.last_mouse_pos.0, my - self.last_mouse_pos.1);

        if let Some(mut inventory) = self.ui_handler.get_mut("inventory") {
            if let Ok(mut player) = self.ecs.get::<&Player>(self.player) {
                if player.action_state.key("toggle_inventory") {
                    inventory.visible = !inventory.visible;
                }
            }
        }

        let ui_hovered = self.ui_handler.update(event_pump, self.screen_size);
        // self.last_mouse_pos = (mx, my);

        ui_hovered
    }
}
