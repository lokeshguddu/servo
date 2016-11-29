use webvr::*;

use ipc_channel::ipc::IpcSender;
use msg::constellation_msg::PipelineId;

pub type WebVRResult<T> = Result<T, String>;

#[derive(Deserialize, Serialize)]
pub enum WebVRMsg {
    RegisterContext(PipelineId),
    UnregisterContext(PipelineId),
    PollEvents(IpcSender<bool>),
    GetVRDisplays(IpcSender<WebVRResult<Vec<VRDisplayData>>>),
    GetFrameData(PipelineId, u64, f64, f64, IpcSender<WebVRResult<VRFrameData>>),
    ResetPose(PipelineId, u64, IpcSender<WebVRResult<VRDisplayData>>),
    RequestPresent(PipelineId, u64, IpcSender<WebVRResult<()>>),
    ExitPresent(PipelineId, u64, Option<IpcSender<WebVRResult<()>>>),
    Exit,
}