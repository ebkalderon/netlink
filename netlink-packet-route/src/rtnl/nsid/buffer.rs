use crate::{
    rtnl::nla::{NlaBuffer, NlasIterator},
    DecodeError,
};

pub const NSID_HEADER_LEN: usize = 4;

buffer!(MessageBuffer(NSID_HEADER_LEN) {
    rtgen_family: (u8, 0),
    payload: (slice, NSID_HEADER_LEN..),
});

impl<'a, T: AsRef<[u8]> + ?Sized> MessageBuffer<&'a T> {
    pub fn nlas(&self) -> impl Iterator<Item = Result<NlaBuffer<&'a [u8]>, DecodeError>> {
        NlasIterator::new(self.payload())
    }
}
