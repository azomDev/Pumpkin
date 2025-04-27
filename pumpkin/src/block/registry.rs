use crate::block::pumpkin_block::{BlockMetadata, PumpkinBlock};
use crate::entity::player::Player;
use crate::server::Server;
use crate::world::{BlockFlags, World};
use pumpkin_data::fluid::Fluid;
use pumpkin_data::item::Item;
use pumpkin_data::{Block, BlockState};
use pumpkin_inventory::OpenContainer;
use pumpkin_protocol::server::play::SUseItemOn;
use pumpkin_util::math::position::BlockPos;
use pumpkin_world::BlockStateId;
use pumpkin_world::block::BlockDirection;
use std::collections::HashMap;
use std::sync::Arc;

use super::pumpkin_fluid::PumpkinFluid;

pub enum BlockActionResult {
    /// Allow other actions to be executed
    Continue,
    /// Block other actions
    Consume,
}

#[derive(Default)]
pub struct BlockRegistry {
    blocks: HashMap<Vec<String>, Arc<dyn PumpkinBlock>>,
    fluids: HashMap<Vec<String>, Arc<dyn PumpkinFluid>>,
}

impl BlockRegistry {
    pub fn register<T: PumpkinBlock + BlockMetadata + 'static>(&mut self, block: T) {
        self.blocks.insert(block.names(), Arc::new(block));
    }

    pub fn register_fluid<T: PumpkinFluid + BlockMetadata + 'static>(&mut self, fluid: T) {
        self.fluids.insert(fluid.names(), Arc::new(fluid));
    }

    pub async fn on_use(
        &self,
        block: &Block,
        player: &Player,
        location: BlockPos,
        server: &Server,
        world: &Arc<World>,
    ) {
        let pumpkin_block = self.get_pumpkin_block(block);
        if let Some(pumpkin_block) = pumpkin_block {
            pumpkin_block
                .normal_use(block, player, location, server, world)
                .await;
        }
    }

    pub async fn explode(&self, block: &Block, world: &Arc<World>, location: BlockPos) {
        let pumpkin_block = self.get_pumpkin_block(block);
        if let Some(pumpkin_block) = pumpkin_block {
            pumpkin_block.explode(block, world, location).await;
        }
    }

    pub async fn use_with_item(
        &self,
        block: &Block,
        player: &Player,
        location: BlockPos,
        item: &Item,
        server: &Server,
        world: &Arc<World>,
    ) -> BlockActionResult {
        let pumpkin_block = self.get_pumpkin_block(block);
        if let Some(pumpkin_block) = pumpkin_block {
            return pumpkin_block
                .use_with_item(block, player, location, item, server, world)
                .await;
        }
        BlockActionResult::Continue
    }

    pub async fn use_with_item_fluid(
        &self,
        fluid: &Fluid,
        player: &Player,
        location: BlockPos,
        item: &Item,
        server: &Server,
        world: &Arc<World>,
    ) -> BlockActionResult {
        let pumpkin_fluid = self.get_pumpkin_fluid(fluid);
        if let Some(pumpkin_fluid) = pumpkin_fluid {
            return pumpkin_fluid
                .use_with_item(fluid, player, location, item, server, world)
                .await;
        }
        BlockActionResult::Continue
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn on_place(
        &self,
        server: &Server,
        world: &World,
        block: &Block,
        face: &BlockDirection,
        block_pos: &BlockPos,
        use_item_on: &SUseItemOn,
        player: &Player,
        other: bool,
    ) -> BlockStateId {
        let pumpkin_block = self.get_pumpkin_block(block);
        if let Some(pumpkin_block) = pumpkin_block {
            return pumpkin_block
                .on_place(
                    server,
                    world,
                    block,
                    face,
                    block_pos,
                    use_item_on,
                    player,
                    other,
                )
                .await;
        }
        block.default_state_id
    }

    pub async fn player_placed(
        &self,
        world: &Arc<World>,
        block: &Block,
        state_id: u16,
        pos: &BlockPos,
        face: &BlockDirection,
        player: &Player,
    ) {
        let pumpkin_block = self.get_pumpkin_block(block);
        if let Some(pumpkin_block) = pumpkin_block {
            pumpkin_block
                .player_placed(world, block, state_id, pos, face, player)
                .await;
        }
    }

    pub async fn can_place_at(
        &self,
        world: &World,
        block: &Block,
        block_pos: &BlockPos,
        face: &BlockDirection,
    ) -> bool {
        let pumpkin_block = self.get_pumpkin_block(block);
        if let Some(pumpkin_block) = pumpkin_block {
            return pumpkin_block.can_place_at(world, block_pos, face).await;
        }
        true
    }

    pub async fn on_placed(
        &self,
        world: &Arc<World>,
        block: &Block,
        state_id: BlockStateId,
        block_pos: &BlockPos,
        old_state_id: BlockStateId,
        notify: bool,
    ) {
        let pumpkin_block = self.get_pumpkin_block(block);
        if let Some(pumpkin_block) = pumpkin_block {
            pumpkin_block
                .placed(world, block, state_id, block_pos, old_state_id, notify)
                .await;
        }
    }

    pub async fn on_placed_fluid(
        &self,
        world: &Arc<World>,
        fluid: &Fluid,
        state_id: BlockStateId,
        block_pos: &BlockPos,
        old_state_id: BlockStateId,
        notify: bool,
    ) {
        let pumpkin_fluid = self.get_pumpkin_fluid(fluid);
        if let Some(pumpkin_fluid) = pumpkin_fluid {
            pumpkin_fluid
                .placed(world, fluid, state_id, block_pos, old_state_id, notify)
                .await;
        }
    }

    pub async fn broken(
        &self,
        world: Arc<World>,
        block: &Block,
        player: &Player,
        location: BlockPos,
        server: &Server,
        state: BlockState,
    ) {
        let pumpkin_block = self.get_pumpkin_block(block);
        if let Some(pumpkin_block) = pumpkin_block {
            pumpkin_block
                .broken(block, player, location, server, world.clone(), state)
                .await;
        }
    }

    pub async fn close(
        &self,
        block: &Block,
        player: &Player,
        location: BlockPos,
        server: &Server,
        container: &mut OpenContainer,
    ) {
        let pumpkin_block = self.get_pumpkin_block(block);
        if let Some(pumpkin_block) = pumpkin_block {
            pumpkin_block
                .close(block, player, location, server, container)
                .await;
        }
    }

    pub async fn on_state_replaced(
        &self,
        world: &Arc<World>,
        block: &Block,
        location: BlockPos,
        old_state_id: BlockStateId,
        moved: bool,
    ) {
        let pumpkin_block = self.get_pumpkin_block(block);
        if let Some(pumpkin_block) = pumpkin_block {
            pumpkin_block
                .on_state_replaced(world, block, location, old_state_id, moved)
                .await;
        }
    }

    /// Updates state of all neighbors of the block
    pub async fn post_process_state(
        &self,
        world: &Arc<World>,
        location: &BlockPos,
        block: &Block,
        flags: BlockFlags,
    ) {
        let state = world.get_block_state(location).await.unwrap();
        for direction in BlockDirection::all() {
            let neighbor_pos = location.offset(direction.to_offset());
            let neighbor_state = world.get_block_state(&neighbor_pos).await.unwrap();
            let pumpkin_block = self.get_pumpkin_block(block);
            if let Some(pumpkin_block) = pumpkin_block {
                let new_state = pumpkin_block
                    .get_state_for_neighbor_update(
                        world,
                        block,
                        state.id,
                        location,
                        &direction.opposite(),
                        &neighbor_pos,
                        neighbor_state.id,
                    )
                    .await;
                world.set_block_state(&neighbor_pos, new_state, flags).await;
            }
        }
    }

    pub async fn prepare(
        &self,
        world: &Arc<World>,
        block_pos: &BlockPos,
        block: &Block,
        state_id: BlockStateId,
        flags: BlockFlags,
    ) {
        let pumpkin_block = self.get_pumpkin_block(block);
        if let Some(pumpkin_block) = pumpkin_block {
            pumpkin_block
                .prepare(world, block_pos, block, state_id, flags)
                .await;
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get_state_for_neighbor_update(
        &self,
        world: &World,
        block: &Block,
        state: BlockStateId,
        block_pos: &BlockPos,
        direction: &BlockDirection,
        neighbor_pos: &BlockPos,
        neighbor_state: BlockStateId,
    ) -> BlockStateId {
        let pumpkin_block = self.get_pumpkin_block(block);
        if let Some(pumpkin_block) = pumpkin_block {
            return pumpkin_block
                .get_state_for_neighbor_update(
                    world,
                    block,
                    state,
                    block_pos,
                    direction,
                    neighbor_pos,
                    neighbor_state,
                )
                .await;
        }
        state
    }

    pub async fn update_neighbors(
        &self,
        world: &Arc<World>,
        block_pos: &BlockPos,
        _block: &Block,
        flags: BlockFlags,
    ) {
        for direction in BlockDirection::abstract_block_update_order() {
            let pos = block_pos.offset(direction.to_offset());

            Box::pin(world.replace_with_state_for_neighbor_update(
                &pos,
                &direction.opposite(),
                flags,
            ))
            .await;
        }
    }

    pub async fn on_neighbor_update(
        &self,
        world: &Arc<World>,
        block: &Block,
        block_pos: &BlockPos,
        source_block: &Block,
        notify: bool,
    ) {
        let pumpkin_block = self.get_pumpkin_block(block);
        if let Some(pumpkin_block) = pumpkin_block {
            pumpkin_block
                .on_neighbor_update(world, block, block_pos, source_block, notify)
                .await;
        }
    }

    #[must_use]
    pub fn get_pumpkin_block(&self, block: &Block) -> Option<&Arc<dyn PumpkinBlock>> {
        self.blocks.iter().find_map(|(ids, pumpkin_block)| {
            ids.contains(&format!("minecraft:{}", block.name))
                .then_some(pumpkin_block)
        })
    }

    #[must_use]
    pub fn get_pumpkin_fluid(&self, fluid: &Fluid) -> Option<&Arc<dyn PumpkinFluid>> {
        self.fluids.iter().find_map(|(ids, pumpkin_block)| {
            ids.contains(&format!("minecraft:{}", fluid.name))
                .then_some(pumpkin_block)
        })
    }

    pub async fn emits_redstone_power(
        &self,
        block: &Block,
        state: &BlockState,
        direction: &BlockDirection,
    ) -> bool {
        let pumpkin_block = self.get_pumpkin_block(block);
        if let Some(pumpkin_block) = pumpkin_block {
            return pumpkin_block
                .emits_redstone_power(block, state, direction)
                .await;
        }
        false
    }

    pub async fn get_weak_redstone_power(
        &self,
        block: &Block,
        world: &World,
        block_pos: &BlockPos,
        state: &BlockState,
        direction: &BlockDirection,
    ) -> u8 {
        let pumpkin_block = self.get_pumpkin_block(block);
        if let Some(pumpkin_block) = pumpkin_block {
            return pumpkin_block
                .get_weak_redstone_power(block, world, block_pos, state, direction)
                .await;
        }
        0
    }

    pub async fn get_strong_redstone_power(
        &self,
        block: &Block,
        world: &World,
        block_pos: &BlockPos,
        state: &BlockState,
        direction: &BlockDirection,
    ) -> u8 {
        let pumpkin_block = self.get_pumpkin_block(block);
        if let Some(pumpkin_block) = pumpkin_block {
            return pumpkin_block
                .get_strong_redstone_power(block, world, block_pos, state, direction)
                .await;
        }
        0
    }
}
