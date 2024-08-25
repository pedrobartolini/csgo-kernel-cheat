use std::mem::MaybeUninit;

use winapi::shared::minwindef::LPVOID;
use winapi::um::fileapi::*;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::ioapiset::DeviceIoControl;
use winapi::um::winioctl::CTL_CODE;
use winapi::um::winioctl::FILE_DEVICE_UNKNOWN;
use winapi::um::winioctl::FILE_SPECIAL_ACCESS;
use winapi::um::winioctl::METHOD_BUFFERED;
use winapi::um::winnt::*;

lazy_static::lazy_static! {
   pub static ref READ: u32 = CTL_CODE(FILE_DEVICE_UNKNOWN, 0x1, METHOD_BUFFERED, FILE_SPECIAL_ACCESS);
   pub static ref WRITE: u32 = CTL_CODE(FILE_DEVICE_UNKNOWN, 0x2, METHOD_BUFFERED, FILE_SPECIAL_ACCESS);
   pub static ref GET_CLIENT: u32 = CTL_CODE(FILE_DEVICE_UNKNOWN, 0x10, METHOD_BUFFERED, FILE_SPECIAL_ACCESS);
   pub static ref GET_ENGINE: u32 = CTL_CODE(FILE_DEVICE_UNKNOWN, 0x11, METHOD_BUFFERED, FILE_SPECIAL_ACCESS);
   pub static ref PING: u32 = CTL_CODE(FILE_DEVICE_UNKNOWN, 0x20, METHOD_BUFFERED, FILE_SPECIAL_ACCESS);
}

#[repr(C)]
pub struct MmCopy {
   addr: u32,
   buff: *mut winapi::ctypes::c_void,
   size: u32
}

pub struct IOCTL {
   mailer:      HANDLE,
   mmcopy_size: u32
}

impl IOCTL {
   pub fn new(registry_path: &str) -> anyhow::Result<Self> {
      let registry_path = std::ffi::CString::new(registry_path)?;

      let mailer = unsafe {
         CreateFileA(
            registry_path.as_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            std::ptr::null_mut(),
            OPEN_EXISTING,
            0,
            std::ptr::null_mut()
         )
      };

      if mailer == INVALID_HANDLE_VALUE {
         return Err(std::io::Error::last_os_error().into());
      }

      Ok(Self {
         mailer,
         mmcopy_size: std::mem::size_of::<MmCopy>() as u32
      })
   }

   fn device_control(&self, protocol: u32, addr: LPVOID, size: u32) -> anyhow::Result<()> {
      if unsafe { DeviceIoControl(self.mailer, protocol, addr, size, addr, size, std::ptr::null_mut(), std::ptr::null_mut()) } == 0 {
         anyhow::bail!("request failed");
      }

      Ok(())
   }

   pub fn get_client(&self) -> anyhow::Result<u32> {
      let client_addr: u32 = 0;
      self.device_control(*GET_CLIENT, &client_addr as *const _ as LPVOID, 4)?;
      Ok(client_addr)
   }

   pub fn get_engine(&self) -> anyhow::Result<u32> {
      let engine_addr: u32 = 0;
      self.device_control(*GET_ENGINE, &engine_addr as *const _ as LPVOID, 4)?;
      Ok(engine_addr)
   }

   pub fn read<T>(&self, addr: u32, size: u32) -> anyhow::Result<T> {
      let buff: MaybeUninit<T> = MaybeUninit::uninit();
      let mmcopy = MmCopy {
         addr,
         size,
         buff: &buff as *const _ as PVOID
      };
      self.device_control(*READ, &mmcopy as *const _ as LPVOID, self.mmcopy_size)?;
      Ok(unsafe { buff.assume_init() })
   }

   pub fn write<T>(&self, addr: u32, data: T, size: u32) -> anyhow::Result<()> {
      let mmcopy = MmCopy {
         addr,
         size,
         buff: &data as *const _ as PVOID
      };
      self.device_control(*WRITE, &mmcopy as *const _ as LPVOID, self.mmcopy_size)?;
      Ok(())
   }
}

impl Drop for IOCTL {
   fn drop(&mut self) {
      if self.mailer != INVALID_HANDLE_VALUE {
         unsafe { winapi::um::handleapi::CloseHandle(self.mailer) };
      }
   }
}
