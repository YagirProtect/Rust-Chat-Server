pub fn get_time_stamp_str() -> String {
    let now = chrono::Local::now();
    let time_stamp = format!("[{}]", now.format("%H:%M:%S"));
    time_stamp
}