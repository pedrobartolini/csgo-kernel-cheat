use math::*;

use super::*;

pub static mut OLD_COMPENSATION: Vec = Vec::new_const(0.0, 0.0, 0.0);

pub fn aimbot(ioctl: &ioctl::IOCTL, client: u32, game_state: u32, local_player: u32) -> anyhow::Result<()> {
   let team: u16 = ioctl.read(local_player + M_I_TEAM_NUM, 2)?;

   let weapon: u16 = ioctl.read(local_player + M_H_ACTIVE_WEAPON, 2)?;
   let weapon_pointer: u32 = ioctl.read(client + DW_ENTITY_LIST + (weapon as u32 - 1) * 16, 4)?;

   if ioctl.read::<u16>(weapon_pointer as u32 + M_I_CLIP1, 2)? == 0 {
      return Ok(());
   }

   // let weapon_id: u16 = ioctl.read(weapon_pointer + M_I_ITEM_DEFINITION_INDEX, 2)?;
   // if WEAPON_IGNORE.contains(&weapon_id) {
   //    return Ok(());
   // }

   let local_eye_position = ioctl.read::<Vec>(local_player + M_VEC_ORIGIN, 12)? + ioctl.read::<Vec>(local_player + M_VEC_VIEW_OFFSET, 12)?;
   let view_angles = ioctl.read::<Vec>(game_state + DW_CLIENT_STATE_VIEW_ANGLES, 12)?;
   let aim_punch = ioctl.read::<Vec>(local_player + M_AIM_PUNCH_ANGLE, 12)? * Vec::new(2.0, 2.0, 2.0);

   let mut best_fov = 2.0;
   let mut best_angle = Vec::new(0.0, 0.0, 0.0);

   for i in 0..32 {
      let player = ioctl.read::<u32>(client + DW_ENTITY_LIST + i * 0x10, 4)?;

      // check teams
      // if ioctl.read::<u16>(player + M_I_TEAM_NUM, 2)? == team {
      //    continue;
      // }

      // check dormant entity
      if ioctl.read::<bool>(player + M_BDORMANT, 1)? {
         continue;
      }

      // check if is alive
      if ioctl.read::<u8>(player + M_I_HEALTH, 1)? == 0 {
         continue;
      }

      // check if its spotted
      if !ioctl.read::<bool>(player + M_B_SPOTTED_BY_MASK, 1)? {
         continue;
      }

      let bone_matrix = ioctl.read::<u32>(player + M_DW_BONE_MATRIX, 4)?;

      const HITBOXES: [u32; 6] = [8, 7, 6, 5, 3, 4];

      for hitbox in HITBOXES {
         let player_head_pos = Vec::new(
            ioctl.read::<f32>(bone_matrix + 0x30 * hitbox + 0x0C, 4)?,
            ioctl.read::<f32>(bone_matrix + 0x30 * hitbox + 0x1C, 4)?,
            ioctl.read::<f32>(bone_matrix + 0x30 * hitbox + 0x2C, 4)?
         );

         let angle = calculate_angle(&local_eye_position, &player_head_pos, &(view_angles + aim_punch));
         let fov = angle.x.hypot(angle.y);

         if fov < best_fov {
            best_fov = fov;
            best_angle = angle;
         }
      }
   }

   let fired: u8 = ioctl.read(local_player + M_I_SHOTS_FIRED, 1)?;

   if fired > 1 {
      if !best_angle.is_zeroed() {
         best_angle.filter(0.01, 0.03);
         ioctl.write(game_state + DW_CLIENT_STATE_VIEW_ANGLES, view_angles + best_angle, 12)?;
      } else {
         let mut new_compensation = unsafe { OLD_COMPENSATION - aim_punch };
         new_compensation.filter(0.25, 0.45);
         ioctl.write(game_state + DW_CLIENT_STATE_VIEW_ANGLES, view_angles + new_compensation, 12)?;
      }
   }

   unsafe {
      if fired > 1 {
         OLD_COMPENSATION = aim_punch;
      } else {
         OLD_COMPENSATION.x = 0.0;
         OLD_COMPENSATION.y = 0.0;
         OLD_COMPENSATION.z = 0.0;
      }
   }

   Ok(())
}
