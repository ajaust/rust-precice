extern crate libc;

use libc::c_char;
use std::ffi::CString;

use precice_bindings


pub struct Precice {
    // This pointer must never be allowed to leave the struct
    dimension: int8
}

impl Precice {
    fn create_solver_interface( participant_name: String, )
    {
        let participant_name = CString::new(participant_name).unwrap();
        precice_bindings::precicec_createSolverInterface( participant_name.as_ptr(), "precice-config.xml", 0, 1 );
    }
}