use pentane::{PentaneSemVer, PentaneUUID, PluginInformation};
use sunset_rs::*;

#[unsafe(no_mangle)]
#[used]
pub static Pentane_PluginInformation: PluginInformation = PluginInformation::new(
    b"Choreo Test",
    b"itsmeft24",
    PentaneUUID::from_str("9e022e9f356e431cab0792939722284f"),
    PentaneSemVer::new(0, 1, 0),
    PentaneSemVer::new(1, 0, 0),
);

#[unsafe(no_mangle)]
#[used]
pub static Pentane_PluginDependencyCount: usize = 0;
// Since we don't depend on any other plugins, we can skip exporting a `Pentane_PluginDependencies`.

pub extern "thiscall" fn get_choreo_name(this: usize, index: i32) -> *const u8 {
	b"choreographies\\cin_oilrig_part1\\cin_oilrig_part1\0".as_ptr()
}

#[unsafe(no_mangle)]
extern "C" fn Pentane_Main() {
    unsafe {
		let restore = sunset_rs::utils::set_permission((0x00acdbc7 + 3) as *mut (), 1, utils::Access::ExecuteReadWrite).unwrap();
        *((0x00acdbc7 + 3) as *mut u8) = 1;
        utils::set_permission((0x00acdbc7 + 3) as *mut (), 1, restore).unwrap();
		sunset_rs::inst::call(0x00acdbda as *mut (), get_choreo_name as *mut ());
    }
}