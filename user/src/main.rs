use offsets::*;

mod aimbot;
mod ioctl;
mod keys;
mod math;
mod offsets;

fn main() -> anyhow::Result<()> {
   Ok(native_dialog::MessageDialog::new()
      .set_type(native_dialog::MessageType::Error)
      .set_title("Erro")
      .set_text(&format!("{:?}", inner().unwrap_err()))
      .show_alert()?)
}

fn inner() -> anyhow::Result<()> {
   let ioctl = ioctl::IOCTL::new("\\\\.\\magicplatearmor")?;

   let client = ioctl.get_client()?;
   let engine = ioctl.get_engine()?;

   loop {
      std::thread::sleep(std::time::Duration::from_millis(1));

      let game_state: u32 = ioctl.read(engine + DW_CLIENT_STATE, 4)?;

      let game_state_state: u32 = ioctl.read(game_state + DW_CLIENT_STATE_STATE, 4)?;
      if game_state_state != 6 {
         continue;
      }

      let local_player: u32 = ioctl.read(client + DW_LOCAL_PLAYER, 4)?;

      let health: u8 = ioctl.read(local_player + M_I_HEALTH, 2)?;
      if health == 0 {
         continue;
      }

      // possibly detected
      // if keys::key_state(keys::VK_SPACE) {
      //    if ioctl.read::<u16>(local_player + M_F_FLAGS, 2)? & (1 << 0) == 0 {
      //       ioctl.write(client + DW_FORCE_JUMP, 4, 4)?;
      //    } else {
      //       ioctl.write(client + DW_FORCE_JUMP, 6, 4)?;
      //    }
      // }

      if ioctl.read::<u8>(client + DW_FORCE_ATTACK, 1)? & (1 << 0) > 0 {
         aimbot::aimbot(&ioctl, client, game_state, local_player)?;
      } else {
         unsafe {
            aimbot::OLD_COMPENSATION.x = 0.0;
            aimbot::OLD_COMPENSATION.y = 0.0;
            aimbot::OLD_COMPENSATION.z = 0.0;
         }
      }
   }
}
