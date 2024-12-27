use chrono::Timelike;

pub fn get_pomodoro_start_time(current_datetime: chrono::DateTime<chrono::Local>) -> chrono::DateTime<chrono::Local> {
    let mut result = current_datetime;

    if result.second() != 0 {
        result += chrono::Duration::minutes(1);
        result = result.with_second(0).unwrap();
    }

    const TIME_STEP: u32 = 5;
    let add_minute = TIME_STEP - (result.minute() % TIME_STEP);
    result += chrono::Duration::minutes(add_minute as i64);

    result
}

pub fn calc_pomodoro(start_datetime: chrono::DateTime<chrono::Local>) -> Vec<chrono::DateTime<chrono::Local>> {
    const REPEAT: i32 = 4;
    const WORK_TIME: i64 = 25;
    const BREAK_TIME: i64 = 5;
    const LONG_BREAK_TIME: i64 = 35;
    let mut result = Vec::new();

    result.push(start_datetime);

    let mut current_datetime = start_datetime;

    for i in 1..=REPEAT {
        current_datetime += chrono::Duration::minutes(WORK_TIME);
        result.push(current_datetime);

        if i == REPEAT {
            continue;
        }
        current_datetime += chrono::Duration::minutes(BREAK_TIME);
        result.push(current_datetime);
    }

    current_datetime += chrono::Duration::minutes(LONG_BREAK_TIME);
    result.push(current_datetime);

    for i in 1..=REPEAT {
        current_datetime += chrono::Duration::minutes(WORK_TIME);
        result.push(current_datetime);

        if i == REPEAT {
            continue;
        }
        current_datetime += chrono::Duration::minutes(BREAK_TIME);
        result.push(current_datetime);
    }

    result
}

pub fn calc_result_pomodoro(res: Vec<chrono::DateTime<chrono::Local>>) -> Vec<String> {
    let mut result = Vec::new();
    let format_datetime = |dt: chrono::DateTime<chrono::Local>| dt.format("%m/%d %H:%M").to_string();
    let format_time = |dt: chrono::DateTime<chrono::Local>| dt.format("%H:%M").to_string();

    let mut work_cnt = 1;
    let mut break_cnt = 1;
    for i in 0..res.len() - 1 {
        if i % 2 == 0 {
            result.push(format!("{} ~ {} work {}", format_datetime(res[i]), format_time(res[i + 1]), work_cnt));
            work_cnt += 1;
        } else {
            result.push(format!("{} ~ {} break {}\n", format_datetime(res[i]), format_time(res[i + 1]), break_cnt));
            break_cnt += 1;
        }
    }

    result
}