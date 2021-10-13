use crate::backend::WaveformResult;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct WaveformNamedResult {
    pub name: String,
    pub result: WaveformResult,
}
