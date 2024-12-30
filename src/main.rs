use pomodoro::*;

#[allow(unused_imports)]
use chrono::TimeZone;

fn main() {
    let current_datetime = chrono::Local::now();
    // 現在の日時を手動で設定する場合は以下のコメントアウトを外し、上記の行をコメントアウトする（例えば明日のある時刻のスケジュールが欲しいときに使う）
    // let current_datetime = chrono::Local
    //     .with_ymd_and_hms(2024, 01, 01, 8, 59, 00)
    //     .unwrap();

    let pomodoro_schedule = get_pomodoro_schedule(current_datetime);
    for schedule in &pomodoro_schedule {
        println!("{}", schedule);
    }
}