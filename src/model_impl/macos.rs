use std::process::Command;

// TODO: do this without external commands
pub fn get_model() -> String {
    let hw_model = Command::new("sysctl")
        .args(["-n", "hw.model"])
        .output()
        .unwrap()
        .stdout;
    let hw_model = String::from_utf8(hw_model).unwrap();

    let kextstat_out = Command::new("kextstat").output().unwrap().stdout;
    let kextstat_out = String::from_utf8(kextstat_out).unwrap();

    if kextstat_out.contains("FakeSMC") || kextstat_out.contains("VirtualSMC") {
        format!("Hackintosh (SMBIOS: {hw_model})")
    } else {
        hw_model
    }
}
