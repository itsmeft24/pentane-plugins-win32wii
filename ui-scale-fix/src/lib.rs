use pentane::{PentaneSemVer, PentaneUUID, PluginInformation};
use sunset_rs::*;

#[unsafe(no_mangle)]
#[used]
pub static Pentane_PluginInformation: PluginInformation = PluginInformation::new(
    b"UI Scale Fix",
    b"itsmeft24",
    PentaneUUID::from_str("ce1e4889425d4c2a8a7d661c479901a6"),
    PentaneSemVer::new(0, 1, 0),
    PentaneSemVer::new(1, 0, 0),
);

#[unsafe(no_mangle)]
#[used]
pub static Pentane_PluginDependencyCount: usize = 0;
// Since we don't depend on any other plugins, we can skip exporting a `Pentane_PluginDependencies`.

#[repr(C)]
pub struct ScaleformViewport {
	pub bufferWidth: u32,
	pub bufferHeight: u32,
	pub left: u32,
	pub top: u32,
	pub width: u32,
	pub height: u32,
	pub scissorLeft: u32,
	pub scissorRight: u32,
	pub scissorWidth: u32,
	pub scissorHeight: u32,
	pub _unk: f32,
	pub _unk2: f32,
	pub flags: u32,
}

#[sunset_rs::hook(offset = 0x010e1d9e, inline)]
pub extern "_cdecl" fn adjust_scaleform_viewport(ctx: &mut InlineCtx) {
	unsafe {
		let view = std::ptr::with_exposed_provenance_mut::<ScaleformViewport>(ctx.ecx.unsigned_integer as usize);
		if (*view).bufferHeight > 720 {
			(*view)._unk = 720.0_f32 / (*view).bufferHeight as f32;
		}
	}
}

#[unsafe(no_mangle)]
extern "C" fn Pentane_Main() {
    unsafe {
		sunset_rs::install_hooks!(adjust_scaleform_viewport);
    }
}