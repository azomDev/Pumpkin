use async_trait::async_trait;
use pumpkin_data::{
    block_properties::{Attachment, BellLikeProperties, BlockProperties, HorizontalFacing},
    item::Item,
};

use pumpkin_macros::pumpkin_block;
use pumpkin_protocol::server::play::SUseItemOn;
use pumpkin_util::math::position::BlockPos;
use pumpkin_world::{
    BlockStateId,
    block::{BlockDirection, HorizontalFacingExt},
    chunk::TickPriority,
};
use std::sync::Arc;

use crate::block::{Block, registry::BlockActionResult};
use crate::{
    block::pumpkin_block::PumpkinBlock,
    entity::player::Player,
    server::Server,
    world::{BlockFlags, World},
};

use super::block_receives_redstone_power;

#[pumpkin_block("minecraft:bell")]
pub struct BellBlock;

#[async_trait]
impl PumpkinBlock for BellBlock {
    async fn can_place_at(
        &self,
        _world: &World,
        _block_pos: &BlockPos,
        _face: &BlockDirection,
    ) -> bool {
        log::info!("{:?}", _face);
        match _face {
            BlockDirection::Up | BlockDirection::Down => {
                let pos = _block_pos.offset(_face.to_offset());
                let block = _world.get_block_state(&pos).await.unwrap();
                return block.is_solid();
            }
            BlockDirection::North
            | BlockDirection::South
            | BlockDirection::West
            | BlockDirection::East => {
                let pos = _block_pos.offset(_face.to_offset());
                let block = _world.get_block_state(&pos).await.unwrap();

                if block.is_solid() {
                    log::info!("block is solid");
                    return true;
                }

                let below_pos = _block_pos.offset(BlockDirection::Down.to_offset());
                let below_block = _world.get_block_state(&below_pos).await.unwrap();
                if below_block.is_solid() {
                    log::info!("below block is solid");
                    return true;
                }

                let above_pos = _block_pos.offset(BlockDirection::Up.to_offset());
                let above_block = _world.get_block_state(&above_pos).await.unwrap();
                log::info!("above block is {} solid", above_block.is_solid());
                return above_block.is_solid();
            }
        }
    }

    async fn on_place(
        &self,
        _server: &Server,
        world: &World,
        block: &Block,
        _face: &BlockDirection,
        block_pos: &BlockPos,
        _use_item_on: &SUseItemOn,
        player: &Player,
        _other: bool,
    ) -> BlockStateId {
        let mut props = BellLikeProperties::default(block);
        props.facing = player.living_entity.entity.get_horizontal_facing();

        props.attachment = match _face {
            BlockDirection::Up => Attachment::Ceiling,
            BlockDirection::Down => Attachment::Floor,
            BlockDirection::North
            | BlockDirection::South
            | BlockDirection::West
            | BlockDirection::East => {
                let temp: HorizontalFacing = _face.to_horizontal_facing().unwrap();
                props.facing = temp;
                let pos = block_pos.offset(temp.to_offset());
                let block = world.get_block_state(&pos).await.unwrap();
                if block.is_solid() {
                    let opposite_pos = block_pos.offset(temp.opposite().to_offset());
                    let opposite_block = world.get_block_state(&opposite_pos).await.unwrap();
                    // TODO: Check decompiled MC which check is used
                    if opposite_block.is_solid() {
                        Attachment::DoubleWall
                    } else {
                        Attachment::SingleWall
                    }
                } else {
                    let below_pos = block_pos.offset(BlockDirection::Down.to_offset());
                    let below_block = world.get_block_state(&below_pos).await.unwrap();
                    if below_block.is_solid() {
                        Attachment::Floor
                    } else {
                        let above_pos = block_pos.offset(BlockDirection::Up.to_offset());
                        let above_block = world.get_block_state(&above_pos).await.unwrap();
                        if above_block.is_solid() {
                            Attachment::Ceiling
                        } else {
                            unreachable!("because of `can_place_at`");
                        }
                    }
                }
            }
        };

        props.to_state_id(block)
    }

    // async fn use_with_item(
    //     &self,
    //     _block: &Block,
    //     _player: &Player,
    //     location: BlockPos,
    //     _item: &Item,
    //     _server: &Server,
    //     world: &Arc<World>,
    // ) -> BlockActionResult {
    //     // toggle_lever(world, &location).await;
    //     BlockActionResult::Consume
    // }

    // async fn on_neighbor_update(
    //     &self,
    //     world: &Arc<World>,
    //     block: &Block,
    //     block_pos: &BlockPos,
    //     _source_block: &Block,
    //     _notify: bool,
    // ) {
    //     let state = world.get_block_state(block_pos).await.unwrap();
    //     let mut props = BellLikeProperties::from_state_id(state.id, block);
    //     let is_powered = props.powered;
    //     let is_receiving_power = block_receives_redstone_power(world, block_pos).await;

    //     if is_powered != is_receiving_power {
    //         if is_powered {
    //             world
    //                 .schedule_block_tick(block, *block_pos, 4, TickPriority::Normal) // TODO get vanilla TickPriority and tick duration
    //                 .await;
    //         } else {
    //             props.powered = true;
    //             // TODO: play animation/sound
    //             world
    //                 .set_block_state(
    //                     block_pos,
    //                     props.to_state_id(block),
    //                     BlockFlags::NOTIFY_LISTENERS,
    //                 )
    //                 .await;
    //         }
    //     }
    // }

    // async fn on_scheduled_tick(&self, world: &Arc<World>, block: &Block, block_pos: &BlockPos) {
    //     let state = world.get_block_state(block_pos).await.unwrap();
    //     let mut props = BellLikeProperties::from_state_id(state.id, block);
    //     let is_powered = props.powered;
    //     let is_receiving_power = block_receives_redstone_power(world, block_pos).await;

    //     if is_powered && !is_receiving_power {
    //         props.powered = props.powered;
    //         world
    //             .set_block_state(
    //                 block_pos,
    //                 props.to_state_id(block),
    //                 BlockFlags::NOTIFY_LISTENERS,
    //             )
    //             .await;
    //     }
    // }
}
