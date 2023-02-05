use cli_table::{format::Justify, Cell, Table};

use crate::responses::EmeterResponse;

pub fn emeter_to_table(emeter_response: EmeterResponse) -> String {
    let emeter_response = emeter_response.emeter.get_realtime;
    let data = vec![vec![
        emeter_response.voltage_mv.cell().justify(Justify::Right),
        emeter_response.current_ma.cell().justify(Justify::Right),
        emeter_response.power_mw.cell().justify(Justify::Right),
        emeter_response.total_wh.cell().justify(Justify::Right),
    ]]
    .table()
    .title(vec![
        "Voltage".cell().justify(Justify::Center),
        "CurrentMilliAmp".cell().justify(Justify::Center),
        "Power".cell().justify(Justify::Center),
        "TotalWattHours".cell().justify(Justify::Center),
    ]);
    return format!("{}", data.display().unwrap());
}
