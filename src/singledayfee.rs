use chrono::Duration;
use chrono::naive::NaiveDateTime;
use crate::fee;

#[derive(Debug)]
pub(crate) struct SingleDayFee {
    pub(crate) start_time: NaiveDateTime,  // 精確到分鐘的入場時間
    pub(crate) end_time: NaiveDateTime,    // 精確到分鐘的離場時間
    pub(crate) fee: i32,                   // 本日應收取費用
}

fn calculate_fee(start_time: NaiveDateTime, end_time: NaiveDateTime) -> Vec<SingleDayFee> {
    assert!(end_time >= start_time);

    let mut result: Vec<SingleDayFee> = Vec::new();
  
    // same day
    if start_time.date() == end_time.date() {
        let start_time = start_time;
        let end_time = end_time;
        result.push(SingleDayFee {
            start_time,
            end_time,
            fee: fee::calculate_fee(start_time.time(), end_time.time()),
        });
        return result;
    }

    // push start day
    let startday_start_time = start_time;
    let startday_end_time = start_time.date().and_hms(23, 59, 59);
    result.push(SingleDayFee {
        start_time: startday_start_time,
        end_time: startday_end_time,
        fee: fee::calculate_fee(startday_start_time.time(), startday_end_time.time()),
    });

    // push middle days
    let mut day = start_time.date() + Duration::days(1);
    while day < end_time.date() {
        let middle_start_time = day.and_hms(0, 0, 0);
        let middle_end_time = day.and_hms(23, 59, 59);
        result.push(SingleDayFee {
            start_time: middle_start_time,
            end_time: middle_end_time,
            fee: fee::calculate_fee(middle_start_time.time(), middle_end_time.time()),
        });
        day += Duration::days(1);
    }

    // push end day
    let endday_start_time = end_time.date().and_hms(0, 0, 0);
    let endday_end_time = end_time;
    result.push(SingleDayFee {
        start_time: endday_start_time,
        end_time: endday_end_time,
        fee: fee::calculate_fee(endday_start_time.time(), endday_end_time.time()),
    });

    result
}

pub(crate) struct SingleDayFeeCalculator {
    start_time: NaiveDateTime,  // 精確到分鐘的入場時間
    end_time: NaiveDateTime,    // 精確到分鐘的離場時間
    day: NaiveDateTime,         // 計算用日期
}

impl SingleDayFeeCalculator {
    pub(crate) fn new(start_time: NaiveDateTime, end_time: NaiveDateTime) -> SingleDayFeeCalculator {
        SingleDayFeeCalculator {
            start_time,
            end_time,
            day: start_time
        }
    }
}

impl Iterator for SingleDayFeeCalculator {
    type Item = SingleDayFee;

    fn next(&mut self) -> Option<Self::Item> {
        if self.day.date() > self.end_time.date() {
            return None;
        } else {
            let (day_start_time, day_end_time) = {
                if self.day.date() == self.end_time.date() {
                    (self.day, self.end_time)
                } else if self.day.date() == self.start_time.date() {
                    self.day = self.day.date().and_hms(0, 0, 0);
                    (self.start_time, self.start_time.date().and_hms(23, 59, 59))
                } else {
                    (self.day.date().and_hms(0, 0, 0), self.day.date().and_hms(23, 59, 59))
                }
            };
            self.day += Duration::days(1);
            return Some(SingleDayFee {
                start_time: day_start_time,
                end_time:   day_end_time,
                fee: fee::calculate_fee(day_start_time.time(), day_end_time.time()),
            });
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    fn parse_date_time(date: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(date, "%Y/%m/%d %H:%M:%S").unwrap()
    }

    #[test]
    fn test0() {
        let from_time = parse_date_time("2002/5/1 23:49:00");
        let to_time = parse_date_time("2002/5/1 23:59:59");
        let result = calculate_fee(from_time, to_time); 
        println!("{:?}", result);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].fee, 0);

        let calculator = SingleDayFeeCalculator::new(from_time, to_time);
        let result = calculator.collect::<Vec<SingleDayFee>>();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].fee, 0);
    }

    #[test]
    fn test1() {
        let from_time = parse_date_time("2002/5/1 23:49:00");
        let to_time = parse_date_time("2002/5/2 00:10:59");
        let result = calculate_fee(from_time, to_time); 
        println!("{:?}", result);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].fee, 0);
        assert_eq!(result[1].fee, 0);

        let calculator = SingleDayFeeCalculator::new(from_time, to_time);
        let result = calculator.collect::<Vec<SingleDayFee>>();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].fee, 0);
        assert_eq!(result[1].fee, 0);
    }
    
    
    #[test]
    fn test2() {
        let from_time = parse_date_time("2002/5/1 23:48:00");
        let to_time = parse_date_time("2002/5/2 00:11:59");
        let result = calculate_fee(from_time, to_time); 
        println!("{:?}", result);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].fee, 7);
        assert_eq!(result[1].fee, 7);

        let calculator = SingleDayFeeCalculator::new(from_time, to_time);
        let result = calculator.collect::<Vec<SingleDayFee>>();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].fee, 7);
        assert_eq!(result[1].fee, 7);
    }

    #[test]
    fn test3() {
        let from_time = parse_date_time("2002/5/1 23:48:00");
        let to_time = parse_date_time("2002/5/3 00:11:59");
        let result = calculate_fee(from_time, to_time); 
        println!("{:?}", result);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].fee, 7);
        assert_eq!(result[1].fee, 50);
        assert_eq!(result[2].fee, 7);

        let calculator = SingleDayFeeCalculator::new(from_time, to_time);
        let result = calculator.collect::<Vec<SingleDayFee>>();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].fee, 7);
        assert_eq!(result[1].fee, 50);
        assert_eq!(result[2].fee, 7);
    }
}