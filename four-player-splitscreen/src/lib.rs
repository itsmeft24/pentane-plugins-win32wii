use pentane::{PentaneSemVer, PentaneUUID, PluginInformation};
use sunset_rs::*;

#[unsafe(no_mangle)]
#[used]
pub static Pentane_PluginInformation: PluginInformation = PluginInformation::new(
    b"4 Player SplitScreen",
    b"itsmeft24",
    PentaneUUID::from_str("e7fb479138de419f88b3639a093b452a"),
    PentaneSemVer::new(0, 1, 1),
    PentaneSemVer::new(1, 0, 0),
);

#[unsafe(no_mangle)]
#[used]
pub static Pentane_PluginDependencyCount: usize = 0;
// Since we don't depend on any other plugins, we can skip exporting a `Pentane_PluginDependencies`.

#[unsafe(no_mangle)]
extern "C" fn Pentane_Main() {
    unsafe {
        let restore = utils::set_permission(0x0048fee9 as *mut (), 1, utils::Access::ExecuteReadWrite).unwrap();
        *(0x0048fee9 as *mut u8) = 0xEB;
        utils::set_permission(0x0048fee9 as *mut (), 1, restore).unwrap();
		
		let restore = utils::set_permission(0x0048e0e7 as *mut (), 1, utils::Access::ExecuteReadWrite).unwrap();
        *(0x0048e0e7 as *mut u8) = 0x03;
        utils::set_permission(0x0048e0e7 as *mut (), 1, restore).unwrap();
		
		let restore = utils::set_permission(0x004876B8 as *mut (), 1, utils::Access::ExecuteReadWrite).unwrap();
        *(0x004876B8 as *mut u8) = 0x01;
        utils::set_permission(0x004876B8 as *mut (), 1, restore).unwrap();

		let restore = utils::set_permission(0x007AE6C3 as *mut (), 1, utils::Access::ExecuteReadWrite).unwrap();
        *(0x007AE6C3 as *mut u8) = 0x04;
        utils::set_permission(0x007AE6C3 as *mut (), 1, restore).unwrap();
		
		inst::nop(0x0048FF81 as *mut (), 5);
    }
}