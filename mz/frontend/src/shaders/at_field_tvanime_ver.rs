use wgpu::*;
use super::{
    base::fragment,
    Shader,
};

pub struct ShaderWork;

impl Shader for ShaderWork {
    async fn run(canvas: zoon::web_sys::HtmlCanvasElement) {
        fragment::run_with(
            include_wgsl!("./at_field_tvanime_ver.wgsl"),
            canvas,
        ).await
    }
}