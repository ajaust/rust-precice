extern crate libc;

mod precice_bindings;
//pub mod precice;

//use precice::libc::c_char;
use precice::libc::{c_int, c_double};
use std::ffi::CString;
use std::ffi::CStr;

//Do not implement part with comminicator here
//pub fn create_solver_interface_with_communicatorparticipant_name: String, config_file_name: String, rank: i8, size: i8, ???


fn create_rust_string_from_c_str( string_to_convert: *const ::std::os::raw::c_char ) -> String
{
    let c_str: &CStr = unsafe { CStr::from_ptr(string_to_convert) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();
    str_buf
}

//let test = unsafe{ precice_bindings::precicec_getVersionInformation() };
//static version_information : String = create_rust_string_from_c_str(unsafe{ precice_bindings::precicec_getVersionInformation() });
//static version_information : String;

pub fn create_solver_interface( participant_name: String, config_file_name: String, rank: i32, size: i32 )
{
    let participant_name = CString::new(participant_name).unwrap();
    let config_file_name = CString::new(config_file_name).unwrap();
    unsafe { precice_bindings::precicec_createSolverInterface( participant_name.as_ptr(), config_file_name.as_ptr(), rank as c_int, size as c_int ) };
}

pub fn precice_initialize() -> f64
{
    //version_information = create_rust_string_from_c_str(unsafe{ precice_bindings::precicec_getVersionInformation() });
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
    let read_data_is_available: i32 = unsafe {  precice_bindings::precicec_isReadDataAvailable() };
    read_data_is_available != 0
}

pub fn is_write_data_required( computed_time_step_length: f64 ) -> bool
{
    let write_data_is_available: i32 = unsafe {  precice_bindings::precicec_isWriteDataRequired( computed_time_step_length ) };
    write_data_is_available != 0
}

pub fn is_time_window_complete() -> bool
{
    let time_window_is_complete: i32 = unsafe {  precice_bindings::precicec_isTimeWindowComplete() };
    time_window_is_complete != 0
}

pub fn has_to_evaluate_surrogate_model() -> bool
{
    let surrogate_model_has_to_be_evaluated: i32 = unsafe {  precice_bindings::precicec_hasToEvaluateSurrogateModel() };
    surrogate_model_has_to_be_evaluated != 0
}

pub fn has_to_evaluate_fine_model() -> bool
{
    let fine_model_has_to_be_evaluated: i32 = unsafe {  precice_bindings::precicec_hasToEvaluateFineModel() };
    fine_model_has_to_be_evaluated != 0
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
    let action_is_required: i32 = unsafe {  precice_bindings::precicec_isActionRequired( action.as_cstring().as_ptr() ) };
    action_is_required != 0
}

pub fn mark_action_fulfilled( action: PreciceActions )
{
    unsafe{ precice_bindings::precicec_markActionFulfilled( action.as_cstring().as_ptr() ) };
}

pub fn has_mesh( mesh_name: String ) -> bool
{
    let mesh_name = CString::new(mesh_name).unwrap();
    let has_mesh_with_name: i32 = unsafe{ precice_bindings::precicec_hasMesh( mesh_name.as_ptr() ) };
    has_mesh_with_name != 0
}

pub fn get_mesh_id( mesh_name: String) -> i32
{
    let mesh_name = CString::new(mesh_name).unwrap();
    let mesh_id = unsafe{ precice_bindings::precicec_getMeshID( mesh_name.as_ptr() ) };
    mesh_id
}

pub fn set_mesh_vertex( mesh_id: i32, position: &[f64]) -> i32
{
    let vertex_id = unsafe{ precice_bindings::precicec_setMeshVertex( mesh_id as c_int, position.as_ptr() ) };
    vertex_id
}

pub fn get_mesh_vertex_size( mesh_id: i32 ) -> i32
{
    let number_of_mesh_vertexes = unsafe{ precice_bindings::precicec_getMeshVertexSize( mesh_id as c_int ) };
    number_of_mesh_vertexes
}

pub fn set_mesh_vertices( mesh_id: i32, positions: &[f64], ids: &mut [i32] )
{
    unsafe { precice_bindings::precicec_setMeshVertices( mesh_id as c_int, ids.len() as c_int, positions.as_ptr(), ids.as_mut_ptr() ) };
}

pub fn get_mesh_vertices( mesh_id: i32, ids: &[i32], positions: &mut [f64] )
{
    unsafe { precice_bindings::precicec_getMeshVertices( mesh_id as c_int, ids.len() as c_int, ids.as_ptr(), positions.as_mut_ptr(), ) };
}


pub fn get_mesh_vertex_ids_from_positions( mesh_id: i32, positions: &[f64], ids: &mut [i32] )
{
    unsafe { precice_bindings::precicec_getMeshVertexIDsFromPositions( mesh_id as c_int, ids.len() as c_int, positions.as_ptr(), ids.as_mut_ptr() ) };
}

pub fn set_mesh_edge( mesh_id: i32, first_vertex_id: i32, second_vertex_id: i32) -> i32
{
    let edge_id: i32 = unsafe { precice_bindings::precicec_setMeshEdge( mesh_id as c_int, first_vertex_id as c_int, second_vertex_id as c_int ) };
    edge_id
}

pub fn set_mesh_triangle( mesh_id: i32, first_edge_id: i32, second_edge_id: i32, third_edge_id: i32 )
{
    unsafe { precice_bindings::precicec_setMeshTriangle( mesh_id as c_int, first_edge_id as c_int, second_edge_id as c_int, third_edge_id as c_int ) };
}


pub fn set_mesh_triangle_with_edges( mesh_id: i32, first_vertex_id: i32, second_vertex_id: i32, third_vertex_id: i32 )
{
    unsafe { precice_bindings::precicec_setMeshTriangleWithEdges( mesh_id as c_int, first_vertex_id as c_int, second_vertex_id as c_int, third_vertex_id as c_int ) };
}

pub fn set_mesh_quad( mesh_id: i32, first_edge_id: i32, second_edge_id: i32, third_edge_id: i32, fourth_edge_id: i32 )
{
    unsafe { precice_bindings::precicec_setMeshQuad( mesh_id as c_int, first_edge_id as c_int, second_edge_id as c_int, third_edge_id as c_int, fourth_edge_id as c_int  ) };
}

pub fn set_mesh_quad_with_edges( mesh_id: i32, first_vertex_id: i32, second_vertex_id: i32, third_vertex_id: i32, fourth_vertex_id: i32 )
{
    unsafe { precice_bindings::precicec_setMeshQuadWithEdges( mesh_id as c_int, first_vertex_id as c_int, second_vertex_id as c_int, third_vertex_id as c_int, fourth_vertex_id as c_int ) };
}

pub fn has_data( data_name: String, mesh_id: i32) -> bool
{
    let data_name = CString::new(data_name).unwrap();
    let has_data_with_name: i32 = unsafe {  precice_bindings::precicec_hasData( data_name.as_ptr(), mesh_id as c_int ) };
    has_data_with_name != 0
}

pub fn get_data_id( data_name: String, mesh_id: i32) -> i32
{
    let data_name = CString::new(data_name).unwrap();
    let data_id: i32 = unsafe {  precice_bindings::precicec_getDataID( data_name.as_ptr(), mesh_id as c_int ) };
    data_id
}

pub fn map_read_data_to( to_mesh_id: i32)
{
    unsafe {  precice_bindings::precicec_mapReadDataTo( to_mesh_id as c_int ) };
}

pub fn map_write_data_to( from_mesh_id: i32)
{
    unsafe {  precice_bindings::precicec_mapWriteDataFrom( from_mesh_id as c_int ) };
}

pub fn write_block_vector_data( data_id: i32, value_indices: &[i32], values: &[f64] )
{
    unsafe { precice_bindings::precicec_writeBlockVectorData( data_id as c_int, value_indices.len() as c_int, value_indices.as_ptr(), values.as_ptr() ) };
}

pub fn write_vector_data( data_id: i32, value_index: i32, values: &[f64] )
{
    unsafe { precice_bindings::precicec_writeVectorData( data_id as c_int, value_index as c_int, values.as_ptr() ) };
}

pub fn write_block_scalar_data( data_id: i32, value_indices: &[i32], values: &[f64] )
{
    unsafe { precice_bindings::precicec_writeBlockScalarData( data_id as c_int, value_indices.len() as c_int, value_indices.as_ptr(), values.as_ptr() ) };
}

pub fn write_scalar_data( data_id: i32, value_index: i32, value: f64 )
{
    unsafe { precice_bindings::precicec_writeScalarData( data_id as c_int, value_index as c_int, value as c_double ) };
}

pub fn read_block_vector_data( data_id: i32, value_indices: &[i32], values: &mut [f64] )
{
    unsafe { precice_bindings::precicec_readBlockVectorData( data_id as c_int, value_indices.len() as c_int, value_indices.as_ptr(), values.as_mut_ptr() ) };
}

pub fn read_vector_data( data_id: i32, value_index: i32, values: &mut [f64] )
{
    unsafe { precice_bindings::precicec_readVectorData( data_id as c_int, value_index as c_int, values.as_mut_ptr() ) };
}


pub fn read_block_scalar_data( data_id: i32, value_indices: &[i32], values: &mut [f64] )
{
    unsafe { precice_bindings::precicec_readBlockScalarData( data_id as c_int, value_indices.len() as c_int, value_indices.as_ptr(), values.as_mut_ptr() ) };
}

pub fn read_scalar_data( data_id: i32, value_index: i32, value: &mut f64 )
{
    unsafe { precice_bindings::precicec_readScalarData( data_id as c_int, value_index as c_int, value ) };
}

pub fn get_version_information() -> String
{
    let version_string = create_rust_string_from_c_str( unsafe{ precice_bindings::precicec_getVersionInformation() } );
    version_string
}