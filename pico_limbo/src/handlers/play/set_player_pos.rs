use crate::handlers::play::set_player_position_and_rotation::teleport_player_to_spawn_out_of_bounds;
use crate::server::batch::Batch;
use crate::server::client_state::ClientState;
use crate::server::packet_handler::{PacketHandler, PacketHandlerError};
use crate::server::packet_registry::PacketRegistry;
use crate::server_state::ServerState;
use minecraft_packets::play::set_player_position_packet::SetPlayerPositionPacket;
use minecraft_protocol::prelude::VarInt;
use std::time::Instant;

use minecraft_packets::play::transfer_packet::TransferPacket;

impl PacketHandler for SetPlayerPositionPacket {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch<PacketRegistry>, PacketHandlerError> {
        // AFK-mode movement detection
        let mut batch = Batch::new();
        if server_state.afk_mode().enabled && !server_state.afk_mode().return_host.is_empty() {
            let (prev_x, _prev_y, prev_z) = client_state.get_position();
            let dx = self.x - prev_x;
            let dz = self.z - prev_z;
            let dist = (dx * dx + dz * dz).sqrt();
            if dist > server_state.afk_mode().move_distance {
                let now = Instant::now();
                if let Some(prev_time) = client_state.note_movement() {
                    if now.duration_since(prev_time).as_millis() as u64 >= server_state.afk_mode().move_threshold_ms {
                        // trigger transfer via Velocity (use configured host string)
                        let packet = TransferPacket {
                            host: server_state.afk_mode().return_host.clone(),
                            port: VarInt::from(server_state.afk_mode().return_port),
                        };
                        batch.queue(|| PacketRegistry::Transfer(packet));
                        client_state.clear_move_timer();
                        return Ok(batch);
                    }
                } else {
                    client_state.set_move_instant(now);
                }
            } else {
                client_state.clear_move_timer();
            }
        }

        // update position and handle out-of-bounds teleporting
        let mut out_batch = teleport_player_to_spawn_out_of_bounds(
            client_state,
            server_state,
            self.feet_y,
        );
        // ensure we store X/Z/Y position
        client_state.set_position(self.x, self.feet_y, self.z);
        // merge batches
        batch.extend(out_batch);
        Ok(batch)
    }
}
