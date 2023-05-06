pub(crate) use crate::utils::error::block_parsing_error::BlockParsingError;
pub(crate) use crate::utils::error::decompile_error::DecompileError;
pub(crate) use crate::utils::error::io_error::IoError;
pub(crate) use crate::utils::error::FireballError;
#[allow(unused_imports)]
pub(crate) use log::{debug, error, info, trace, warn};

pub(crate) type BitBox = bitvec::prelude::BitBox<u16>;
pub(crate) type BitSlice = bitvec::prelude::BitSlice<u16>;
