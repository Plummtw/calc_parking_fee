use std::cmp::min;
use chrono::naive::NaiveTime;

pub(crate) fn fee(minutes: i32) -> i32 {
  let hours = minutes / 60;
  let minutes = minutes % 60;
  let fee = match (hours, minutes) {
      (0, 0..=10) => 0,
      (h, 0) => h * 10,
      (h, 1..=30) => h * 10 + 7,
      (h, 31..=59) => h * 10 + 10,
      _ => panic!("Cannot calculate fee")
  };
  min(fee, 50)
}

pub(crate) fn calculate_fee(from_time: NaiveTime, to_time: NaiveTime) -> i32{
    let duration = to_time - from_time;
    fee(duration.num_minutes() as i32)
}

pub(crate) fn calculate_fee_str(from_time: &str, to_time: &str) -> i32{
    let from_time = NaiveTime::parse_from_str(from_time, "%H:%M:%S").unwrap();
    let to_time = NaiveTime::parse_from_str(to_time, "%H:%M:%S").unwrap();
    calculate_fee(from_time, to_time)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testcase(from_time: &str, to_time: &str, expected: i32) {
        assert_eq!(calculate_fee_str(from_time, to_time), expected);
    }

    #[test]
    fn tests() {
        testcase("09:00:00", "09:00:00", 0); // [0,10]
        testcase("09:00:00", "09:10:59", 0); // [0,10]
        testcase("09:00:00", "09:11:59", 7); // [11,30]
        testcase("09:00:00", "09:30:59", 7); // [11,30]
        testcase("09:00:00", "09:31:59", 10); // [31,59]
        testcase("09:00:00", "09:59:59", 10); // [31,59]
        testcase("09:00:00", "10:00:59", 10); // 整點
        testcase("09:00:00", "11:00:59", 20); // 整點
        testcase("09:00:00", "12:00:59", 30); // 整點
        testcase("09:00:00", "13:00:59", 40); // 整點
        testcase("09:00:00", "14:00:59", 50); // 整點

        testcase("09:00:00", "10:01:59", 17); // 大於60分,1小時又剩餘<=30
        testcase("09:00:00", "10:30:59", 17); // 大於60分,1小時又剩餘<=30
        testcase("09:00:00", "11:01:59", 27); // 大於60分,2小時又剩餘<=30
        testcase("09:00:00", "11:30:59", 27); // 大於60分,2小時又剩餘<=30
        testcase("09:00:00", "12:01:59", 37); // 大於60分,3小時又剩餘<=30
        testcase("09:00:00", "12:30:59", 37); // 大於60分,3小時又剩餘<=30
        testcase("09:00:00", "13:01:59", 47); // 大於60分,4小時又剩餘<=30
        testcase("09:00:00", "13:30:59", 47); // 大於60分,4小時又剩餘<=30

        testcase("09:00:00", "10:31:59", 20); // 大於60分,1小時又剩餘 >30
        testcase("09:00:00", "10:59:59", 20); // 大於60分,1小時又剩餘 >30
        testcase("09:00:00", "11:31:59", 30); // 大於60分,2小時又剩餘 >30
        testcase("09:00:00", "11:59:59", 30); // 大於60分,2小時又剩餘 >30
        testcase("09:00:00", "12:31:59", 40); // 大於60分,3小時又剩餘 >30
        testcase("09:00:00", "12:59:59", 40); // 大於60分,3小時又剩餘 >30
        testcase("09:00:00", "13:31:59", 50); // 大於60分,4小時又剩餘 >30
        testcase("09:00:00", "13:59:59", 50); // 大於60分,4小時又剩餘 >30

        testcase("09:00:00", "14:01:59", 50); // 每天最多收50元
        testcase("00:00:00", "23:59:59", 50); // 每天最多收50元
    }
}