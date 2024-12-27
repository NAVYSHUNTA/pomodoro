use pomodoro::get_pomodoro_start_time;

fn main() {
    let current_datetime = chrono::Local::now();
    let start_datetime = get_pomodoro_start_time(current_datetime);
    let res = pomodoro::calc_pomodoro(start_datetime);
    let result = pomodoro::calc_result_pomodoro(res);
    for i in 0..result.len() {
        println!("{}", result[i]);
    }
}