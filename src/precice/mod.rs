extern crate libc;

mod precice_bindings;
//pub mod precice;

//use precice::libc::c_char;
use precice::libc::{c_int, c_char};
use std::ffi::CString;
use std::ffi::CStr;

//Do not implement part with comminicator here
//pub fn create_solver_interface_with_communicatorparticipant_name: String, config_file_name: String, rank: i8, size: i8, ???

pub fn create_solver_interface( participant_name: String, config_file_name: String, rank: i32, size: i32 )
{
    let participant_name = CString::new(participant_name).unwrap();
    let config_file_name = CString::new(config_file_name).unwrap();
    unsafe { precice_bindings::precicec_createSolverInterface( participant_name.as_ptr(), config_file_name.as_ptr(), rank as c_int, size as c_int ) };
}

pub fn precice_initialize() -> f64
{
    let dt = unsafe { precice_bindings::precicec_initialize() };
    dt
}

pub fn initialize_data()
{
    unsafe { precice_bindings::precicec_initialize_data() };
}

pub fn advance( computed_time_step_length: f64 ) -> f64
{
    let dt = unsafe {
         precice_bindings::precicec_advance(computed_time_step_length)
    };
    dt
}

pub fn finalize()
{
    unsafe { precice_bindings::precicec_finalize() };
}

pub fn get_dimensions() -> i32
{
    let dim: i32 = unsafe { precice_bindings::precicec_getDimensions() };
    dim
}

pub fn is_coupling_ongoing() -> bool
{
    let coupling_is_ongoing: i32 = unsafe {  precice_bindings::precicec_isCouplingOngoing() };
    coupling_is_ongoing != 0
}

pub fn is_read_data_available() -> bool
{
    let coupling_is_ongoing: i32 = unsafe {  precice_bindings::precicec_isReadDataAvailable() };
    coupling_is_ongoing != 0
}

pub fn is_write_data_required( computed_time_step_length: f64 ) -> bool
{
    let coupling_is_ongoing: i32 = unsafe {  precice_bindings::precicec_isWriteDataRequired( computed_time_step_length ) };
    coupling_is_ongoing != 0
}

pub fn is_time_window_complete() -> bool
{
    let coupling_is_ongoing: i32 = unsafe {  precice_bindings::precicec_isTimeWindowComplete() };
    coupling_is_ongoing != 0
}

pub fn has_to_evaluate_surrogate_model() -> bool
{
    let coupling_is_ongoing: i32 = unsafe {  precice_bindings::precicec_hasToEvaluateSurrogateModel() };
    coupling_is_ongoing != 0
}

pub fn has_to_evaluate_fine_model() -> bool
{
    let coupling_is_ongoing: i32 = unsafe {  precice_bindings::precicec_hasToEvaluateFineModel() };
    coupling_is_ongoing != 0
}

pub enum PreciceActions {
    WriteIterationCheckpoint,
    ReadIterationCheckpoint,
    WriteInitialData,
}

impl PreciceActions {
    pub fn as_cstring(&self) -> CString {
        match *self {
            PreciceActions::WriteIterationCheckpoint => CString::new("write-iteration-checkpoint").unwrap(),
            PreciceActions::ReadIterationCheckpoint =>  CString::new("read-iteration-checkpoint").unwrap(),
            PreciceActions::WriteInitialData =>  CString::new("write-initial-data").unwrap(),
        }
    }
}

pub fn is_action_required( action: PreciceActions ) -> bool
{
    let coupling_is_ongoing: i32 = unsafe {  precice_bindings::precicec_isActionRequired( action.as_cstring().as_ptr() ) };
    coupling_is_ongoing != 0
}

pub fn mark_action_fulfilled( action: PreciceActions ) -> bool
{
    precicec_markActionFulfilled;
}