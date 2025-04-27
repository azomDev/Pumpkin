use std::sync::Arc;

use async_trait::async_trait;
use pumpkin_data::tag::Tagable;
use pumpkin_data::{
    Block,
    block_properties::{BlockProperties, CactusLikeProperties, EnumVariants, Integer0To15},
};
use pumpkin_macros::pumpkin_block;
use pumpkin_world::BlockStateId;
use pumpkin_world::block::BlockDirection;
use pumpkin_world::chunk::TickPriority;

use crate::block::pumpkin_block::PumpkinBlock;
use crate::world::BlockFlags;
use crate::world::World;
use pumpkin_util::math::position::BlockPos;

#[pumpkin_block("minecraft:cactus")]
pub struct CactusBlock;

#[async_trait]
impl PumpkinBlock for CactusBlock {
    async fn on_scheduled_tick(&self, world: &Arc<World>, _block: &Block, pos: &BlockPos) {
        if !self.can_place_at(world, pos, &BlockDirection::Down).await {
            world.break_block(pos, None, BlockFlags::empty()).await;
        }
    }

    async fn random_tick(&self, block: &Block, world: &Arc<World>, pos: &BlockPos) {
        if world.get_block_state(&pos.up()).await.unwrap().is_air() {
            let state_id = world
                .get_block_state(pos)
                .await
                .expect("`location` should be a cactus")
                .id;
            let age = CactusLikeProperties::from_state_id(state_id, block).age;
            if age == Integer0To15::L15 {
                world
                    .set_block_state(&pos.up(), state_id, BlockFlags::empty())
                    .await;
                let props = CactusLikeProperties {
                    age: Integer0To15::L0,
                };
                world
                    .set_block_state(pos, props.to_state_id(block), BlockFlags::empty())
                    .await;
            } else {
                let props = CactusLikeProperties {
                    age: Integer0To15::from_index(age.to_index() + 1),
                };
                world
                    .set_block_state(pos, props.to_state_id(block), BlockFlags::empty())
                    .await;
            }
        }
    }

    async fn get_state_for_neighbor_update(
        &self,
        world: &World,
        block: &Block,
        state: BlockStateId,
        pos: &BlockPos,
        _direction: &BlockDirection,
        _neighbor_pos: &BlockPos,
        _neighbor_state: BlockStateId,
    ) -> BlockStateId {
        if !self.can_place_at(world, pos, &BlockDirection::Down).await {
            world
                .schedule_block_tick(block, *pos, 1, TickPriority::Normal)
                .await;
        }
        state
    }

    async fn can_place_at(&self, world: &World, pos: &BlockPos, _face: &BlockDirection) -> bool {
        // TODO: use tags
        // Disallow to place any blocks nearby a cactus
        for direction in BlockDirection::horizontal() {
            let (block, state) = world
                .get_block_and_block_state(&pos.offset(direction.to_offset()))
                .await
                .unwrap();
            if state.is_solid() || block == Block::LAVA {
                return false;
            }
        }
        let block = world.get_block(&pos.down()).await.unwrap();
        // TODO: use tags
        (block == Block::CACTUS || block.is_tagged_with("minecraft:sand").unwrap())
            && !world.get_block_state(&pos.up()).await.unwrap().is_liquid()
    }
}
