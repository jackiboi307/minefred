use crate::gameobjtype::base::*;
use crate::types::*;
use crate::gameobjtype::types::TYPES;

use std::collections::HashMap;

macro_rules! setter {
    ($name:ident, $type:ty) => {
        pub const fn $name(&mut self, value: $type) -> &mut Self {
            self.$name = Some(value);
            self
        }
    };
}

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
//

pub struct UpdateData {
    // pub events: EventState,
    // TODO delta time
}

#[derive(Copy, Clone)]
pub struct GameObjectTypeBuilder {
    pub key: &'static str,
    pub init: Option<InitFnType>,
    pub update: Option<UpdateFnType>,
    pub texture: Option<&'static str>,
}

impl GameObjectTypeBuilder {
    pub const fn new(key: &'static str) -> Self {
        Self {
            key,
            init: None,
            update: None,
            texture: None,
        }
    }

    setter!(init, InitFnType);
    setter!(update, UpdateFnType);
    setter!(texture, &'static str);
}

pub struct GameObjectType {
    pub update_fn_id: Option<UpdateFnIdType>,
    pub texture: Option<&'static str>,
}

pub struct GameObjectTypes {
    types: Box<[GameObjectType]>, // indexed by GameObjectTypeId
    init_fns: Box<[Option<InitFnType>]>, // indexed by GameObjectTypeId
    update_fns: Box<[UpdateFnType]>, // indexed by UpdateFnIdType
    key_id_map: HashMap<&'static str, GameObjectTypeId>,
}

impl GameObjectTypes {
    pub fn generate() -> Self {
        let mut types = Vec::new();
        let mut update_fns = Vec::new();
        let mut init_fns = Vec::new();
        let mut key_id_map = HashMap::new();

        for builder in TYPES {
            let id = TryInto::<GameObjectTypeId>::try_into(types.len()).expect(
                "too many game objects for id type");
            key_id_map.insert(builder.key, id);
            init_fns.push(builder.init);

            let update_fn_id = if let Some(update_fn) = builder.update {
                Some({
                    let id = if !update_fns.contains(&update_fn) {
                        update_fns.push(update_fn);
                        update_fns.len() - 1
                    } else {
                        update_fns.iter().position(|x|
                            std::ptr::fn_addr_eq(*x, update_fn)).unwrap()
                    };
                    TryInto::<UpdateFnIdType>::try_into(id).expect(
                        "too many unique update functions for id type")
                })
            } else {
                None
            };

            let gameobjtype = GameObjectType {
                update_fn_id,
                texture: builder.texture,
            };

            types.push(gameobjtype);
        }

        Self{
            types: types.into(),
            init_fns: init_fns.into(),
            update_fns: update_fns.into(),
            key_id_map,
        }
    }

    pub fn from_id(&self, id: GameObjectTypeId) -> &GameObjectType {
        self.types.get(id as usize).expect("invalid id")
    }

    pub fn get_id(&self, key: &'static str) -> GameObjectTypeId {
        *self.key_id_map.get(key).expect(
            &format!("invalid key '{}'", key).to_string())
    }

    pub fn init_entity
            <'a>(&self, entity_builder: &'a mut EntityBuilder, key: &'static str)
            -> Result<(), Error> {

        let id = self.get_id(key);

        if let Some(init) = self.init_fns.get(self.get_id(key) as usize).unwrap() {
            init(entity_builder)?;
        }

        entity_builder.add(GameObjectTypeComponent{id});

        if let Some(id) =
                self.types.get(
                    *self.key_id_map.get(&key).expect("invalid key") as usize
                ).unwrap().update_fn_id {

            entity_builder.add(UpdateFn{id});
        }

        Ok(())
    }

    pub fn get_update_fn_from_id(&self, id: UpdateFnIdType) -> UpdateFnType {
        self.update_fns[id as usize]
    }
}
