use chrono::{Local, TimeZone};
use pomodoro::*;

#[test]
fn test_setup_time() {
    let start_datetime = Local
        .with_ymd_and_hms(2024, 12, 27, 12, 27, 33)
        .unwrap();
    assert_eq!("2024-12-27 12:27:33 +09:00", start_datetime.to_string());
}

#[test]
fn test_get_pomodoro_start_time_less_than_60_seconds() {
    let current_datetime = Local
        .with_ymd_and_hms(2024, 12, 27, 12, 59, 01)
        .unwrap();
let result = get_pomodoro_start_time(current_datetime);

assert_eq!("2024-12-27 13:05:00 +09:00", result.to_string());
}

#[test]
fn test_get_pomodoro_start_time_exactly_60_seconds() {
    let current_datetime = Local
        .with_ymd_and_hms(2024, 12, 27, 12, 59, 00)
        .unwrap();
    let result = get_pomodoro_start_time(current_datetime);

    assert_eq!("2024-12-27 13:00:00 +09:00", result.to_string());
}

#[test]
fn test_get_pomodoro_start_time_greater_than_60_seconds() {
    let current_datetime = Local
        .with_ymd_and_hms(2024, 12, 27, 12, 58, 59)
        .unwrap();
    let result = get_pomodoro_start_time(current_datetime);

    assert_eq!("2024-12-27 13:00:00 +09:00", result.to_string());
}

#[test]
fn test_calc_pomodoro() {
    let start_datetime = Local
        .with_ymd_and_hms(2024, 12, 27, 12, 30, 00)
        .unwrap();
    let result = calc_pomodoro(start_datetime);
    assert_eq!("2024-12-27 12:30:00 +09:00", result[0].to_string());
    assert_eq!("2024-12-27 12:55:00 +09:00", result[1].to_string());

    assert_eq!("2024-12-27 13:00:00 +09:00", result[2].to_string());
    assert_eq!("2024-12-27 13:25:00 +09:00", result[3].to_string());

    assert_eq!("2024-12-27 13:30:00 +09:00", result[4].to_string());
    assert_eq!("2024-12-27 13:55:00 +09:00", result[5].to_string());

    assert_eq!("2024-12-27 14:00:00 +09:00", result[6].to_string());
    assert_eq!("2024-12-27 14:25:00 +09:00", result[7].to_string());

    assert_eq!("2024-12-27 15:00:00 +09:00", result[8].to_string());
    assert_eq!("2024-12-27 15:25:00 +09:00", result[9].to_string());

    assert_eq!("2024-12-27 15:30:00 +09:00", result[10].to_string());
    assert_eq!("2024-12-27 15:55:00 +09:00", result[11].to_string());

    assert_eq!("2024-12-27 16:00:00 +09:00", result[12].to_string());
    assert_eq!("2024-12-27 16:25:00 +09:00", result[13].to_string());

    assert_eq!("2024-12-27 16:30:00 +09:00", result[14].to_string());
    assert_eq!("2024-12-27 16:55:00 +09:00", result[15].to_string());

    assert_eq!(16, result.len());
}

#[test]
fn test_calc_result_pomodoro() {
    let start_datetime = Local
        .with_ymd_and_hms(2024, 12, 27, 12, 30, 00)
        .unwrap();
    let result = calc_result_pomodoro(calc_pomodoro(start_datetime));
    assert_eq!("12/27 12:30 ~ 12:55 work 1", result[0]);
    assert_eq!("12/27 12:55 ~ 13:00 break 1\n", result[1]);

    assert_eq!("12/27 13:00 ~ 13:25 work 2", result[2]);
    assert_eq!("12/27 13:25 ~ 13:30 break 2\n", result[3]);

    assert_eq!("12/27 13:30 ~ 13:55 work 3", result[4]);
    assert_eq!("12/27 13:55 ~ 14:00 break 3\n", result[5]);

    assert_eq!("12/27 14:00 ~ 14:25 work 4", result[6]);
    assert_eq!("12/27 14:25 ~ 15:00 break 4\n", result[7]);

    assert_eq!("12/27 15:00 ~ 15:25 work 5", result[8]);
    assert_eq!("12/27 15:25 ~ 15:30 break 5\n", result[9]);

    assert_eq!("12/27 15:30 ~ 15:55 work 6", result[10]);
    assert_eq!("12/27 15:55 ~ 16:00 break 6\n", result[11]);

    assert_eq!("12/27 16:00 ~ 16:25 work 7", result[12]);
    assert_eq!("12/27 16:25 ~ 16:30 break 7\n", result[13]);

    assert_eq!("12/27 16:30 ~ 16:55 work 8", result[14]);

    assert_eq!(15, result.len());
}