use anyhow::{anyhow, Error, Result};
use regex::Regex;
use std::{fs::read_to_string, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct DateTime {
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct Event {
    time: DateTime,
    kind: EventKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Shift {
    guard_id: u16,
    events: Vec<Event>,
}

impl Shift {
    fn sleep_summary(&self) -> [u16; 60] {
        let mut summary = [0; 60];
        let mut fell_asleep = 0;
        for event in &self.events {
            match event.kind {
                EventKind::FellAsleep => {
                    if event.time.hour == 23 {
                        fell_asleep = 0;
                    } else {
                        fell_asleep = event.time.minute
                    }
                }
                EventKind::WakeUp => {
                    for i in fell_asleep..event.time.minute {
                        summary[i as usize] = 1;
                    }
                }
                _ => {}
            }
        }
        summary
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Guard {
    id: u16,
    sleep_summary: Vec<[u16; 60]>,
}
#[derive(Debug, PartialEq, Eq)]
enum EventKind {
    FellAsleep,
    WakeUp,
    Start(u16),
}

impl FromStr for Event {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let reg: Regex = Regex::new(
            r"(?x)
                \[
                    (?P<year>\d{1,4})-(?P<month>\d{1,2})-(?P<day>\d{1,2})
                        \s+
                    (?P<hour>\d{1,2}):(?P<minute>\d{1,2})
                \]
                \s+
                (?:Guard\ \#(?P<guard_id>\d{1,4})\ begins\ shift|(?P<message>.+))",
        )
        .unwrap();

        let caps = match reg.captures(s) {
            None => return Err(anyhow!("unrecognized event")),
            Some(caps) => caps,
        };

        let datetime = DateTime {
            month: caps["month"].parse()?,
            day: caps["day"].parse()?,
            hour: caps["hour"].parse()?,
            minute: caps["minute"].parse()?,
        };

        let kind = if let Some(guard_id) = caps.name("guard_id") {
            EventKind::Start(guard_id.as_str().parse()?)
        } else if &caps["message"] == "falls asleep" {
            EventKind::FellAsleep
        } else if &caps["message"] == "wakes up" {
            EventKind::WakeUp
        } else {
            return Err(anyhow!("Wrong event kind"));
        };

        Ok(Event {
            time: datetime,
            kind: kind,
        })
    }
}

fn group_shifts_by_guard(shifts: Vec<Shift>) -> Vec<Guard> {
    let mut guards: Vec<Guard> = Vec::new();
    for shift in shifts {
        let guard = match guards.iter_mut().find(|g| g.id == shift.guard_id) {
            Some(guard) => guard,
            None => {
                guards.push(Guard {
                    id: shift.guard_id,
                    sleep_summary: Vec::new(),
                });
                guards.last_mut().unwrap()
            }
        };
        guard.sleep_summary.push(shift.sleep_summary());
    }
    guards
}

fn get_sorted_events(content: &str) -> Result<Vec<Event>> {
    let mut events: Vec<Event> = content
        .lines()
        .map(|line| Event::from_str(line))
        .collect::<Result<Vec<Event>>>()?;
    events.sort_by(|a, b| {
        a.time
            .month
            .cmp(&b.time.month)
            .then_with(|| a.time.day.cmp(&b.time.day))
            .then_with(|| a.time.hour.cmp(&b.time.hour))
            .then_with(|| a.time.minute.cmp(&b.time.minute))
    });
    Ok(events)
}

fn group_events_by_shifts(events: Vec<Event>) -> Vec<Shift> {
    let mut shifts: Vec<Shift> = Vec::new();
    let mut shift: Shift = Shift {
        guard_id: 0,
        events: Vec::new(),
    };

    for event in events {
        match event.kind {
            EventKind::Start(id) => {
                if !shift.events.is_empty() {
                    shifts.push(shift);
                }
                shift = Shift {
                    guard_id: id,
                    events: Vec::new(),
                };
            }
            _ => shift.events.push(event),
        }
    }
    shifts.push(shift);
    shifts
}

fn find_guard_with_most_sleep(guards: Vec<Guard>) -> u16 {
    let mut max_sleep: u16 = 0;
    let mut guard_id = 0;
    for guard in guards {
        let sleep = guard
            .sleep_summary
            .iter()
            .flatten()
            .map(|v| u16::from(*v))
            .sum();
        if sleep > max_sleep {
            max_sleep = sleep;
            guard_id = guard.id;
        }
    }
    guard_id
}

fn get_minute_most_asleep(guard: &Guard) -> (usize, u16) {
    (0..60)
        .map(|i| (i, guard.sleep_summary.iter().map(|v| v[i]).sum::<u16>()))
        .max_by_key(|x| x.1)
        .unwrap()
}

pub fn solve() {
    let content = read_to_string("inputs/Year2018/Day4.txt").unwrap();
    let events = get_sorted_events(&content).unwrap();
    let shifts = group_events_by_shifts(events);
    let guards = group_shifts_by_guard(shifts);
    let most_asleep_guard = find_guard_with_most_sleep(guards.clone());
    let minute_most_asleep =
        get_minute_most_asleep(guards.iter().find(|g| g.id == most_asleep_guard).unwrap());
    let minute_total_most_asleep = guards
        .into_iter()
        .map(|g| {
            let minute = get_minute_most_asleep(&g);
            (g.id, minute)
        })
        .max_by(|a, b| a.1 .1.cmp(&b.1 .1))
        .unwrap();
    println!(
        "{:#?}",
        minute_total_most_asleep.0 as usize * minute_total_most_asleep.1 .0
    );
}

#[cfg(test)]
mod test {
    use super::*;

    static content: &str = r#"[1518-11-22 23:47] Guard #2969 begins shift
                         [1518-05-03 00:27] wakes up
                         [1518-05-03 00:02] Guard #349 begins shift
                         [1518-05-03 00:24] falls asleep
                         [1518-11-22 23:53] falls asleep
                         [1518-11-23 00:20] falls asleep
                         [1518-11-23 00:12] wakes up
                         [1518-11-23 00:42] wakes up"#;

    #[test]
    fn test_parse_event() {
        assert_eq!(
            Event::from_str("[1518-05-03 00:27] wakes up").unwrap(),
            Event {
                time: DateTime {
                    month: 5,
                    day: 3,
                    hour: 0,
                    minute: 27
                },
                kind: EventKind::WakeUp
            }
        );
        assert_eq!(
            Event::from_str("[1518-11-22 23:47] Guard #2969 begins shift").unwrap(),
            Event {
                time: DateTime {
                    month: 11,
                    day: 22,
                    hour: 23,
                    minute: 47
                },
                kind: EventKind::Start(2969)
            }
        );
    }

    #[test]
    fn test_get_sorted_events() {
        assert_eq!(
            get_sorted_events(content).unwrap(),
            vec![
                Event {
                    time: DateTime {
                        month: 5,
                        day: 3,
                        hour: 0,
                        minute: 2
                    },
                    kind: EventKind::Start(349)
                },
                Event {
                    time: DateTime {
                        month: 5,
                        day: 3,
                        hour: 0,
                        minute: 24
                    },
                    kind: EventKind::FellAsleep
                },
                Event {
                    time: DateTime {
                        month: 5,
                        day: 3,
                        hour: 0,
                        minute: 27
                    },
                    kind: EventKind::WakeUp
                },
                Event {
                    time: DateTime {
                        month: 11,
                        day: 22,
                        hour: 23,
                        minute: 47
                    },
                    kind: EventKind::Start(2969)
                },
                Event {
                    time: DateTime {
                        month: 11,
                        day: 22,
                        hour: 23,
                        minute: 53
                    },
                    kind: EventKind::FellAsleep
                },
                Event {
                    time: DateTime {
                        month: 11,
                        day: 23,
                        hour: 0,
                        minute: 12
                    },
                    kind: EventKind::WakeUp
                },
                Event {
                    time: DateTime {
                        month: 11,
                        day: 23,
                        hour: 0,
                        minute: 20
                    },
                    kind: EventKind::FellAsleep
                },
                Event {
                    time: DateTime {
                        month: 11,
                        day: 23,
                        hour: 0,
                        minute: 42
                    },
                    kind: EventKind::WakeUp
                }
            ]
        );
    }

    #[test]
    fn test_group_events_by_shift() {
        let events = get_sorted_events(content).unwrap();
        assert_eq!(
            group_events_by_shifts(events),
            vec![
                Shift {
                    guard_id: 349,
                    events: vec![
                        Event {
                            time: DateTime {
                                month: 5,
                                day: 3,
                                hour: 0,
                                minute: 24
                            },
                            kind: EventKind::FellAsleep
                        },
                        Event {
                            time: DateTime {
                                month: 5,
                                day: 3,
                                hour: 0,
                                minute: 27
                            },
                            kind: EventKind::WakeUp
                        }
                    ]
                },
                Shift {
                    guard_id: 2969,
                    events: vec![
                        Event {
                            time: DateTime {
                                month: 11,
                                day: 22,
                                hour: 23,
                                minute: 53
                            },
                            kind: EventKind::FellAsleep
                        },
                        Event {
                            time: DateTime {
                                month: 11,
                                day: 23,
                                hour: 0,
                                minute: 12
                            },
                            kind: EventKind::WakeUp
                        },
                        Event {
                            time: DateTime {
                                month: 11,
                                day: 23,
                                hour: 0,
                                minute: 20
                            },
                            kind: EventKind::FellAsleep
                        },
                        Event {
                            time: DateTime {
                                month: 11,
                                day: 23,
                                hour: 0,
                                minute: 42
                            },
                            kind: EventKind::WakeUp
                        }
                    ]
                }
            ]
        );
    }

    #[test]
    fn test_sleep_summary() {
        let events = get_sorted_events(content).unwrap();
        let shifts = group_events_by_shifts(events);
        assert_eq!(
            shifts[0].sleep_summary(),
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ]
        );
        assert_eq!(
            shifts[1].sleep_summary(),
            [
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ]
        );
    }
}
