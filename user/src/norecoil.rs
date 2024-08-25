// use math::*;

// use super::*;

// pub static mut OLD_PUNCH: Vec = Vec::new_const(0.0, 0.0, 0.0);

// pub fn norecoil(ioctl: &ioctl::IOCTL, client: u32, game_state: u32, local_player: u32) -> anyhow::Result<()> {
//    let weapon: u16 = ioctl.read(local_player + M_H_ACTIVE_WEAPON, 2)?;
//    let weapon_pointer: u32 = ioctl.read(client + DW_ENTITY_LIST + (weapon as u32 - 1) * 16, 4)?;

//    let ammo: u16 = ioctl.read(weapon_pointer as u32 + M_I_CLIP1, 2)?;
//    if ammo == 0 {
//       return Ok(());
//    }

//    let view_angle: Vec = ioctl.read(game_state + DW_CLIENT_STATE_VIEW_ANGLES, 12)?;
//    let aim_punch_angle = ioctl.read::<Vec>(local_player + M_AIM_PUNCH_ANGLE, 12)? * Vec::new(2.0, 2.0, 2.0);
//    let fired: u8 = ioctl.read(local_player + M_I_SHOTS_FIRED, 1)?;

//    if fired > 1 {
//       let mut compensation = unsafe { OLD_PUNCH } - aim_punch_angle;
//       // compensation.filter(0.1, 0.5);

//       let mut new_angle = view_angle + compensation;
//       new_angle.normalize();

//       ioctl.write(game_state + DW_CLIENT_STATE_VIEW_ANGLES, new_angle, 12)?;
//    }

//    unsafe { OLD_PUNCH = aim_punch_angle }

//    Ok(())
// }
