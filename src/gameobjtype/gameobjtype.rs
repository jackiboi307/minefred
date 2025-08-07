use crate::gameobjtype::base::*;
use crate::gameobjtype::types::TYPES;
use crate::event::Events;
use crate::types::*;

use std::collections::HashMap;

type InitFnType = fn(
    entity: &mut EntityBuilder) -> Result<&mut EntityBuilder, Error>;

pub type UpdateFnType = fn(
    ecs: &mut ECSWorld,
    ecs_id: ECSEntityId,
    update_data: &UpdateData) -> Result<(), Error>;

// type RenderFnType = fn(
//     ecs: &ECSWorld,
//     ecs_id: ECSEntityId,
//     render_info: &RenderInfo,
//     textures: &Textures,
//     canvas: &mut Canvas) -> Result<(), Error>;

#[derive(Copy, Clone)]
pub struct GameObjectType {
    pub key: &'static str,
    pub init: Option<InitFnType>,
    pub update: Option<UpdateFnType>,
    // pub render: Option<RenderFnType>,
}

pub struct UpdateData {
    pub events: Events,
    // TODO delta time
}

pub struct GameObjectTypes {
    update_fns: Box<[UpdateFnType]>,
    update_fn_keys: HashMap<&'static str, UpdateFnIdType>,
    init_fns: HashMap<&'static str, InitFnType>,
}

impl GameObjectTypes {
    pub fn generate() -> Self {
        let mut update_fns = Vec::new();
        let mut update_fn_keys = HashMap::new();
        let mut init_fns = HashMap::new();

        for gameobjecttype in TYPES {
            if let Some(init_fn) = gameobjecttype.init {
                init_fns.insert(gameobjecttype.key, init_fn);
            }

            if let Some(update_fn) = gameobjecttype.update {
                let index =
                    if !update_fns.contains(&update_fn) {
                        update_fns.push(update_fn);
                        update_fns.len() - 1
                    } else {
                        update_fns.iter().position(|x|
                            std::ptr::fn_addr_eq(*x, update_fn)).unwrap()
                    };

                update_fn_keys.insert(
                    gameobjecttype.key,
                    index.try_into().expect("too many update functions for id type"));
            }
        }

        let update_fns = update_fns.into_boxed_slice();

        Self{
            update_fns,
            update_fn_keys,
            init_fns,
        }
    }

    pub fn init_entity
            <'a>(&self, entity_builder: &'a mut EntityBuilder, key: &'static str)
            -> Result<(), Error> {

        (self.init_fns.get(key).expect(
            &format!("invalid type '{}'", key).to_string()
        ))(entity_builder)?;

        if let Some(id) = self.get_update_fn_id_from_key(key) {
            entity_builder.add(UpdateFn{id});
        }

        Ok(())
    }

    pub fn get_update_fn_from_id(&self, id: UpdateFnIdType) -> UpdateFnType {
        self.update_fns[id as usize]
    }

    fn get_update_fn_id_from_key(&self, key: &'static str) -> Option<UpdateFnIdType> {
        self.update_fn_keys.get(key).copied()
    }
}
