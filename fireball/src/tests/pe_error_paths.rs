use super::hello_world_binary;
use crate::{core::FireRaw, pe::Pe, prelude::FireballError};

#[test]
fn from_binary_rejects_non_pe_input() {
    let err = Pe::from_binary(vec![0x00, 0x01, 0x02, 0x03]).unwrap_err();
    assert!(matches!(err, FireballError::PeParsingFailed(_)));
}

#[test]
fn from_path_propagates_io_error() {
    let err = Pe::from_path("__fireball_missing_file_for_error_test__.exe").unwrap_err();
    assert!(matches!(err, FireballError::IoError(_)));
}

#[test]
fn analyze_all_respects_cancellation_request() {
    let binary = hello_world_binary();
    let pe = Pe::from_binary(binary.to_vec()).unwrap();
    pe.cancel_analysis();

    let err = pe.analyze_all().unwrap_err();
    assert!(matches!(
        err,
        crate::prelude::DecompileError::Unknown(Some(msg)) if msg == "analysis cancelled"
    ));
}
