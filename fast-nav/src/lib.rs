use pentane::{PentaneSemVer, PentaneUUID, PluginInformation};
use sunset_rs::*;

#[unsafe(no_mangle)]
#[used]
pub static Pentane_PluginInformation: PluginInformation = PluginInformation::new(
    b"Fast Menu Navigation",
    b"itsmeft24",
    PentaneUUID::from_str("5a2e54e731ad42a296c9240f3a2fb93a"),
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
		inst::nop(0x010dd7c4 as *mut (), 0xC);
    }
}