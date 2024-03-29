use std::{
    cmp::Ordering,
    fmt::{self},
};

#[derive(Clone)]
enum MatchState {
    WIN,
    LOSE,
    DRAW,
}

#[derive(Eq)]
struct TallyStats {
    team_name: String,
    matches_played: u16,
    won: u16,
    drawn: u16,
    lost: u16,
    points: u16,
}

impl PartialEq for TallyStats {
    fn eq(&self, other: &Self) -> bool {
        return self.team_name.eq(&other.team_name)
            && self.matches_played == other.matches_played
            && self.won == other.won
            && self.drawn == other.drawn
            && self.lost == other.lost
            && self.points == other.points;
    }
}

impl PartialOrd for TallyStats {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.points.cmp(&other.points) {
            Ordering::Greater => Some(Ordering::Less),
            Ordering::Less => Some(Ordering::Greater),
            Ordering::Equal => Some(self.team_name.cmp(&other.team_name)),
        }
    }
}

impl Ord for TallyStats {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.points.cmp(&other.points) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => self.team_name.cmp(&other.team_name),
        }
    }
}

impl fmt::Display for TallyStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut spaces = String::from("");

        for _i in 0..(31 - self.team_name.len()) {
            spaces.push(' ');
        }

        write!(
            f,
            "{}{}|  {} |  {} |  {} |  {} |  {}",
            &self.team_name,
            &spaces,
            self.matches_played,
            self.won,
            self.drawn,
            self.lost,
            self.points
        )
    }
}

impl TallyStats {
    fn to_string(&self) -> String {
        let mut spaces = String::from("");

        for _i in 0..(31 - self.team_name.len()) {
            spaces.push(' ');
        }

        let str = format!(
            "{}{}|  {} |  {} |  {} |  {} |  {}",
            &self.team_name,
            &spaces,
            self.matches_played,
            self.won,
            self.drawn,
            self.lost,
            self.points
        );

        return str;
    }
}

pub fn parse_lines(match_results: &str) -> Vec<String> {
    let mut all_lines: Vec<String> = Vec::new();

    let as_bytes = match_results.as_bytes();

    let mut start_index: usize = 0;

    for index in 0..as_bytes.len() {
        if as_bytes[index] == b'\n' || index == as_bytes.len() - 1 {
            let line_vec = match index == as_bytes.len() - 1 {
                true => Vec::from(&as_bytes[start_index..]),
                false => Vec::from(&as_bytes[start_index..index]),
            };
            let line = String::from_utf8(line_vec).unwrap();

            all_lines.push(line);
            start_index = index + 1;
        }
    }

    return all_lines;
}

pub fn parse_semi_color_separated_values(raw_str: String) -> Vec<String> {
    let mut values_vec: Vec<String> = Vec::new();
    let as_bytes = raw_str.as_bytes();

    let mut index: usize = 0;

    for i in 0..as_bytes.len() {
        if as_bytes[i] == b';' || i == as_bytes.len() - 1 {
            let value = match i == as_bytes.len() - 1 {
                true => Vec::from(&raw_str[index..]),
                false => Vec::from(&raw_str[index..i]),
            };
            let utf8_str = String::from_utf8(value).unwrap();
            values_vec.push(utf8_str);
            index = i + 1;
        }
    }

    return values_vec;
}

fn parse_match_result(match_result: &String) -> Option<MatchState> {
    if match_result.eq(&String::from("win")) {
        return Some(MatchState::WIN);
    }

    if match_result.eq(&String::from("loss")) {
        return Some(MatchState::LOSE);
    }

    if match_result.eq(&String::from("draw")) {
        return Some(MatchState::DRAW);
    }

    return None;
}

fn update_team_team(team_stats: &mut TallyStats, match_state: MatchState) {
    team_stats.matches_played = team_stats.matches_played + 1;
    match match_state {
        MatchState::WIN => {
            team_stats.won = team_stats.won + 1;
            team_stats.points = team_stats.points + 3;
        }
        MatchState::LOSE => {
            team_stats.lost = team_stats.lost + 1;
        }
        MatchState::DRAW => {
            team_stats.drawn = team_stats.drawn + 1;
            team_stats.points = team_stats.points + 1;
        }
    }
}

fn team_two_state(team_one_state: MatchState) -> MatchState {
    match team_one_state {
        MatchState::DRAW => MatchState::DRAW,
        MatchState::LOSE => MatchState::WIN,
        MatchState::WIN => MatchState::LOSE,
    }
}

fn table_header() -> String {
    return String::from("Team                           | MP |  W |  D |  L |  P");
}

pub fn tally(match_results: &str) -> String {
    let lines = parse_lines(match_results);
    let mut teams: Vec<TallyStats> = Vec::new();

    for line in lines.iter() {
        let vals = parse_semi_color_separated_values(line.clone());

        let team_one_name = &vals[0];
        let team_two_name = &vals[1];
        let team_one_state = parse_match_result(&vals[2]).unwrap();

        match teams
            .iter_mut()
            .find(|team| team.team_name.eq(team_one_name))
        {
            Some(team) => {
                update_team_team(team, team_one_state.clone());
            }
            None => {
                let mut new_team_one = TallyStats {
                    team_name: String::clone(&team_one_name),
                    matches_played: 0,
                    won: 0,
                    drawn: 0,
                    points: 0,
                    lost: 0,
                };

                update_team_team(&mut new_team_one, team_one_state.clone());
                teams.push(new_team_one);
            }
        };

        match teams
            .iter_mut()
            .find(|team| team.team_name.eq(team_two_name))
        {
            Some(team) => {
                update_team_team(team, team_two_state(team_one_state.clone()));
            }
            None => {
                let mut new_team_two = TallyStats {
                    team_name: String::clone(&team_two_name),
                    matches_played: 0,
                    won: 0,
                    drawn: 0,
                    points: 0,
                    lost: 0,
                };

                update_team_team(&mut new_team_two, team_two_state(team_one_state.clone()));

                teams.push(new_team_two);
            }
        };
    }

    teams.sort();

    let mut table_str = table_header();

    for team_idx in 0..teams.len() {
        let team_str = teams[team_idx].to_string();
        let sttr = format!("\n{}", team_str.as_str());
        table_str.push_str(sttr.as_str());
    }

    return table_str;
}
