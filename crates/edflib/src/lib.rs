use std::{ os::raw::c_int, path::PathBuf, sync::{ Arc, Mutex } };

use anyhow::{ anyhow, Result };
use derive_new::new;
use edflib_sys::*;
use utils::*;

mod utils;

pub enum Filetype {
    EDF,
    BDF,
}

impl Filetype {
    fn from(ext: &str) -> Self {
        match ext {
            "edf" => Filetype::EDF,
            "bdf" => Filetype::BDF,
            _ => Filetype::EDF,
        }
    }
    fn ext(&self) -> &str {
        match self {
            Filetype::EDF => "edf",
            Filetype::BDF => "bdf",
        }
    }
}

#[derive(new)]
struct Inner {
    #[new(value = "0")]
    hdl: i32,
    #[new(value = "Filetype::EDF")]
    filetype: Filetype,
}

#[derive(new)]
pub struct Edf {
    path: PathBuf,
    #[new(value = "Arc::new(Mutex::new(Inner::new()))")]
    inner: Arc<Mutex<Inner>>,
    #[new(value = "1")]
    pub number_of_signals: i32,
}

impl Edf {
    pub fn open_file_writeonly(&self) -> Result<()> {
        let path = PathBuf::from(self.path.to_str().unwrap());
        let ext = path.extension().unwrap().to_str().unwrap();
        let filetype = Filetype::from(ext);

        let path = str_to_char(path.to_str().unwrap());
        let mut inner = self.inner.lock().unwrap();

        let filetype = match filetype {
            Filetype::EDF => EDFLIB_FILETYPE_EDFPLUS as c_int,
            Filetype::BDF => EDFLIB_FILETYPE_BDFPLUS as c_int,
        };
        let hdl = unsafe { edfopen_file_writeonly(path, filetype, self.number_of_signals) };
        inner.hdl = hdl;

        if hdl < 0 {
            let msg = format!("Can not open file \"{}\"for writing", self.path.to_str().unwrap());
            Err(anyhow!(msg))
        } else {
            Ok(())
        }
    }
}
