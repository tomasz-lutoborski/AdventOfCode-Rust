use std::{collections::HashMap, fs::read_to_string, str::FromStr};

use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Time {
    month: u8,
    day: u8,
    minute: i8,
}

impl FromStr for Time {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let date_hour = s.split_whitespace().collect_vec();
        if date_hour.len() != 2 {
            return Err(anyhow!("Invalid date-hour format"));
        };

        let date = date_hour[0].split('-').collect_vec();
        let hour = date_hour[1].split(':').collect_vec();

        let month = date[1]
            .parse()
            .map_err(|_| anyhow!("Invalid month format"))?;
        let day = date[2]
            .parse()
            .map_err(|_| anyhow!("Invalid month format"))?;
        let minute = hour[1]
            .parse::<i8>()
            .map_err(|_| anyhow!("Invalid minute format"))?;

        let minute = if hour[0] == "23" { minute - 60 } else { minute };

        Ok(Time { month, day, minute })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Event {
    Start(u16),
    Asleep,
    WakeUp,
}

#[derive(Debug, PartialEq, Eq)]
struct Log {
    time: Time,
    event: Event,
}

impl FromStr for Log {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let time_msg = s.trim_start_matches('[').split("] ").collect_vec();
        if time_msg.len() != 2 {
            return Err(anyhow!("Invalid log format"));
        }
        let time = Time::from_str(time_msg[0])?;
        let msg: (&str, &str) = time_msg[1]
            .split_whitespace()
            .take(2)
            .collect_tuple()
            .ok_or_else(|| anyhow!("Failed to parse event message"))?;

        let event = match msg {
            ("Guard", id) => {
                let id = id
                    .trim_start_matches('#')
                    .parse::<u16>()
                    .map_err(|_| anyhow!("Invalid guard ID"))?;
                Event::Start(id)
            }
            ("wakes", _) => Event::WakeUp,
            ("falls", _) => Event::Asleep,
            _ => return Err(anyhow!("Unknown message")),
        };

        Ok(Log { time, event })
    }
}

type Guards = HashMap<u16, Vec<Vec<i8>>>;

fn group_logs_by_start_event(logs: Vec<Log>) -> Vec<Vec<Log>> {
    let mut groups: Vec<Vec<Log>> = Vec::new();
    let mut current_group: Vec<Log> = Vec::new();

    for log in logs {
        match log.event {
            Event::Start(_) => {
                if !current_group.is_empty() {
                    groups.push(current_group);
                    current_group = Vec::new();
                }
            }
            _ => {}
        }
        current_group.push(log);
    }
    if !current_group.is_empty() {
        groups.push(current_group);
    }

    groups
}

fn get_shifts_logs(s: &str) -> Vec<Vec<Log>> {
    let mut times = s.lines().map(|l| l.parse::<Log>().unwrap()).collect_vec();

    times.sort_unstable_by(|a, b| {
        a.time.month.cmp(&b.time.month).then_with(|| {
            a.time
                .day
                .cmp(&b.time.day)
                .then_with(|| a.time.minute.cmp(&b.time.minute))
        })
    });

    group_logs_by_start_event(times)
}

fn parse_shift(logs: Vec<Log>) -> (u16, Vec<(Event, i8)>) {
    let id = match logs[0].event {
        Event::Start(n) => n,
        _ => 0,
    };

    let events = logs
        .into_iter()
        .map(|log| (log.event, log.time.minute))
        .collect_vec();

    (id, events)
}

fn sum_minutes_asleep_on_shift(shift: Vec<(Event, i8)>) -> i8 {
    let mut sum = 0;
    if shift.len() == 1 {
        return 0;
    };

    for (i, event) in shift.clone().into_iter().enumerate() {
        match event.0 {
            Event::Asleep => {
                sum += (event.1..shift.get(i + 1).unwrap_or(&(Event::WakeUp, 59)).1).len()
            }
            _ => continue,
        }
    }

    sum as i8
}

pub fn solve() {
    let content = read_to_string("inputs/Year2018/Day4.txt").unwrap();

    let logs = get_shifts_logs(&content);
    // let shifts = logs
    //     .into_iter()
    //     .map(|log| parse_shift(log))
    //     .map(|(id, events)| (id, sum_minutes_asleep_on_shift(events)))
    //     .collect_vec();

    println!("{:#?}", logs);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_time() {
        assert_eq!(
            "1518-05-11 00:47".parse::<Time>().unwrap(),
            Time {
                month: 5,
                day: 11,
                minute: 47
            }
        );
        assert_eq!(
            "1518-03-28 23:56".parse::<Time>().unwrap(),
            Time {
                month: 3,
                day: 28,
                minute: -4
            }
        )
    }
}
