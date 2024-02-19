use libwebp_sys::{WebPPicture, WebPPictureFree};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub(crate) struct ManagedPicture(pub(crate) WebPPicture);

impl Drop for ManagedPicture {
    fn drop(&mut self) {
        unsafe { WebPPictureFree(&mut self.0 as _) }
    }
}

impl Deref for ManagedPicture {
    type Target = WebPPicture;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ManagedPicture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
