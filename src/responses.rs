use serde::Deserialize;
#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DefaultResponse {
    pub system: System,
}
#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct System {
    pub set_relay_state: SetRelayState,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetRelayState {
    pub err_code: i32,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct SystemInfo {
    pub system: SysInfo,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct SysInfo {
    pub get_sysinfo: Info,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Info {
    pub relay_state: i32,
}
