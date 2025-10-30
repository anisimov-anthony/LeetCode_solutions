#![allow(dead_code)]
struct TopVotedCandidate {
    leaderboard: Vec<Vec<i32>>,
}

impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut lb = Vec::with_capacity(persons.len());
        let mut votes = std::collections::HashMap::new();

        let mut pts: Vec<_> = persons
            .iter()
            .zip(times.iter())
            .map(|pt| vec![pt.0, pt.1])
            .collect();
        pts.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        for (i, pt) in pts.iter().enumerate() {
            votes
                .entry(pt[0])
                .and_modify(|(cnt, tm): &mut (_, _)| {
                    *cnt += 1;
                    *tm = i;
                })
                .or_insert((1, i));

            let max_votes = votes.iter().map(|(_, en)| en.0).max().unwrap();
            let leader = *votes
                .iter()
                .filter(|(_, en)| en.0 == max_votes)
                .map(|fl| (fl.0, fl.1 .0, fl.1 .1))
                .max_by(|a, b| a.2.cmp(&b.2))
                .unwrap()
                .0;
            lb.push(vec![*pt[1], *leader]);
        }

        Self { leaderboard: lb }
    }

    fn q(&self, t: i32) -> i32 {
        for i in 0..self.leaderboard.len() {
            if self.leaderboard[i][0] == t {
                return self.leaderboard[i][1];
            }
            if self.leaderboard[i][0] > t {
                return self.leaderboard[i - 1][1];
            }
        }
        return self.leaderboard[self.leaderboard.len() - 1][1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_voted_candidate_basic() {
        let tv = TopVotedCandidate::new(vec![0, 1, 1, 0, 0, 1, 0], vec![0, 5, 10, 15, 20, 25, 30]);
        assert_eq!(tv.q(3), 0);
        assert_eq!(tv.q(12), 1);
        assert_eq!(tv.q(25), 1);
        assert_eq!(tv.q(15), 0);
        assert_eq!(tv.q(24), 0);
        assert_eq!(tv.q(28), 1);
    }
}
