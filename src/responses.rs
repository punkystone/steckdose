use serde::Deserialize;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DefaultResponse {
    pub system: System,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct System {
    pub set_relay_state: SetRelayState,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetRelayState {
    pub err_code: i32,
}

#[derive(Deserialize)]
pub struct SystemInfo {
    pub system: SysInfo,
}

#[derive(Deserialize)]
pub struct SysInfo {
    pub get_sysinfo: Info,
}

#[derive(Deserialize)]
pub struct Info {
    pub relay_state: i32,
}
#[derive(Deserialize)]
pub struct Realtime {
    pub voltage_mv: i64,
    pub current_ma: i64,
    pub power_mw: i64,
    pub total_wh: i64,
    pub err_code: i32,
}

#[derive(Deserialize)]
pub struct Emeter {
    pub get_realtime: Realtime,
}

#[derive(Deserialize)]
pub struct EmeterResponse {
    pub emeter: Emeter,
}
