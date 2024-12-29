use advent_of_code_2024::shared::{PartSolution, Parts};
use hashbrown::{HashMap, HashSet};

advent_of_code_2024::solution!(1083, String::from("as,bu,cp,dj,ez,fd,hu,it,kj,nx,pp,xh,yu"));

fn parse_input(input: &str) -> HashMap<String, HashSet<String>> {
    let connections = input
        .trim()
        .lines()
        .map(|l| l.trim().split_once('-').unwrap())
        .collect::<Vec<_>>();

    let mut network = HashMap::new();

    for (from, to) in connections {
        insert_into_network(&mut network, from, to);
        insert_into_network(&mut network, to, from);
    }

    network
}

fn insert_into_network(network: &mut HashMap<String, HashSet<String>>, from: &str, to: &str) {
    network
        .entry(from.into())
        .and_modify(|conn: &mut HashSet<String>| {
            conn.insert(to.into());
        })
        .or_insert_with(|| HashSet::from([to.into()]));
}

fn find_combos_of_3(input: &str) -> PartSolution {
    let connections = parse_input(input);

    let mut connections_of_3 = HashSet::<_>::new();

    let mut visited = HashSet::new();

    for (origin, targets) in &connections {
        if !visited.insert(origin.clone()) {
            continue;
        }
        for (t1, t2) in permutations(targets) {
            if connections[&t1].contains(&t2) {
                let mut combo = [origin.clone(), t1, t2];

                combo.sort_unstable();

                let [c1, c2, c3] = combo;

                connections_of_3.insert((c1, c2, c3));
            }
        }
    }

    let mut has_t = 0u32;

    for (c1, c2, c3) in connections_of_3 {
        if c1.starts_with('t') || c2.starts_with('t') || c3.starts_with('t') {
            has_t += 1;
        }
    }

    has_t.into()
}

fn find_largest(
    connections: &HashMap<String, HashSet<String>>,
    set: &mut Vec<String>,
    remaining_targets: &[String],
    global_largest_set: &mut usize,
) -> Vec<String> {
    let mut local_largest_set = Option::<Vec<String>>::None;

    for i in 0..remaining_targets.len() {
        if set.len() + remaining_targets.len() + 1 < *global_largest_set {
            continue;
        }

        if !set
            .iter()
            .all(|connection| connections[connection].contains(&remaining_targets[i]))
        {
            continue;
        }

        let remaining = &remaining_targets[i];

        set.push(remaining.clone());

        let new_set = find_largest(
            connections,
            set,
            &remaining_targets[i + 1..],
            global_largest_set,
        );

        if new_set.len() > local_largest_set.as_ref().map_or(usize::MIN, Vec::len) {
            *global_largest_set = (new_set.len() + 1).max(*global_largest_set);

            local_largest_set = Some(new_set.clone());
        }

        set.pop();
    }

    local_largest_set.unwrap_or(set.clone())
}

fn find_largest_set(input: &str) -> PartSolution {
    let connections = parse_input(input);

    let mut largest_set = Vec::new();

    let mut largest_len = 0;

    for (origin, targets) in &connections {
        let targets = targets.iter().cloned().collect::<Vec<_>>();

        for i in 0..targets.len() {
            #[expect(clippy::int_plus_one, reason = "+1 is clearer")]
            if targets[i..].len() + 1 <= largest_set.len() {
                // we're not gonna get a larger one here
                continue;
            }

            let target = targets[i].clone();

            let mut new_set = find_largest(
                &connections,
                &mut vec![target.clone()],
                &targets[i + 1..],
                &mut largest_len,
            );

            if new_set.len() + 1 > largest_set.len() {
                new_set.push(origin.clone());

                largest_set = new_set;

                largest_len = largest_set.len();
            }
        }
    }

    largest_set.sort_unstable();
    largest_set.join(",").into()
}

fn permutations(targets: &HashSet<String>) -> Vec<(String, String)> {
    let fixed = targets.iter().collect::<Vec<_>>();

    let mut permutations = vec![];

    for left in 0..fixed.len() {
        for right in left..fixed.len() {
            permutations.push((fixed[left].clone(), fixed[right].clone()));
        }
    }

    permutations
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        find_combos_of_3(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        find_largest_set(input)
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(1083, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(7, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2024::shared::solution::read_file;
        use advent_of_code_2024::shared::{PartSolution, Parts};

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::String("as,bu,cp,dj,ez,fd,hu,it,kj,nx,pp,xh,yu".into()),
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::String("co,de,ka,ta".into()),
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
