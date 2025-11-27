use pentane::{PentaneSemVer, PentaneUUID, PluginInformation};
use sunset_rs::*;

#[unsafe(no_mangle)]
#[used]
pub static Pentane_PluginInformation: PluginInformation = PluginInformation::new(
    b"Restored Attack Minions",
    b"itsmeft24, Callum273B",
    PentaneUUID::from_str("2f2b2627c3ec413180e0a34a466fb732"),
    PentaneSemVer::new(0, 1, 0),
    PentaneSemVer::new(1, 0, 0),
);

#[unsafe(no_mangle)]
#[used]
pub static Pentane_PluginDependencyCount: usize = 0;
// Since we don't depend on any other plugins, we can skip exporting a `Pentane_PluginDependencies`.

#[unsafe(no_mangle)]
extern "C" fn Pentane_Main() {
    unsafe {
        let restore = utils::set_permission(0x01603894 as *mut (), 16, utils::Access::ExecuteReadWrite).unwrap();
        *(0x01603894 as *mut *const u8) = "BG_Yugo\0".as_ptr();
        *(0x01603898 as *mut *const u8) = "BG_Zapor\0".as_ptr();
        *(0x0160389C as *mut *const u8) = "BG_Gremlin_Purple\0".as_ptr();
        *(0x016038A0 as *mut *const u8) = "BG_ExplodingMinion\0".as_ptr();
        utils::set_permission(0x01603894 as *mut (), 16, restore).unwrap();
    }
}