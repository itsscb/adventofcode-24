use std::{collections::HashMap, error::Error, fs};

/// Solves the problem for day 05.
///
/// # Errors
///
/// This function will return an error if the file cannot be read or if the input is invalid.
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn solve_day05(path: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    let (rules, updates) = parse_input(&content);

    let ordered_updates = updates
        .iter()
        .map(|update| order_updates(update, &rules))
        .collect::<Vec<_>>();

    let result_one = updates
        .iter()
        .zip(ordered_updates.iter())
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a.clone())
        .collect::<Vec<_>>();

    let result_two = updates
        .iter()
        .zip(ordered_updates.iter())
        .filter(|(a, b)| a != b)
        .map(|(_, b)| b.clone())
        .collect::<Vec<_>>();
    let part_one = sum_middle_value(&result_one);
    let part_two = sum_middle_value(&result_two);
    Ok((part_one, part_two))
}

fn parse_input(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let parts: Vec<_> = input.split("\n\n").collect();
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    parts[0].split('\n').for_each(|r| {
        let values = r
            .split('|')
            .map(|i| i.parse().unwrap())
            .collect::<Vec<i32>>();

        let index = values[0];
        map.entry(index)
            .or_default()
            .append(values[1..].to_vec().as_mut());
    });
    // .collect();

    let updates: Vec<Vec<i32>> = parts[1]
        .split('\n')
        .map(|r| r.split(',').map(|i| i.parse().unwrap()).collect())
        .collect();

    (map, updates)
}

fn order_updates(updates: &[i32], rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    if !updates.iter().any(|u| rules.contains_key(u)) {
        return updates.to_vec();
    }

    let mut ordered_updates: Vec<i32> = updates.to_vec();

    ordered_updates.sort_by(|a, b| {
        for (key, values) in rules {
            if a == key && values.contains(b) {
                return std::cmp::Ordering::Less;
            }
        }
        std::cmp::Ordering::Equal
    });
    ordered_updates
}

fn sum_middle_value(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .filter_map(|row| {
            if row.is_empty() || row.len() % 2 == 0 {
                None
            } else {
                Some(row[row.len() / 2])
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
29,13,75
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let (rules, updates) = parse_input(input);
        assert_eq!(rules.len(), 6);
        assert_eq!(rules[&47], vec![53, 13, 61, 29]);
        assert_eq!(rules[&53], vec![29, 13]);

        assert_eq!(updates.len(), 6);
        assert_eq!(updates[0], vec![75, 47, 61, 53, 29]);

        let sum = sum_middle_value(&updates);
        assert_eq!(sum, 61 + 53 + 13 + 47 + 13 + 75);

        let want = vec![75, 29, 13];
        let got = order_updates(&updates[2], &rules);
        assert_eq!(got, want);

        let want = vec![97, 75, 47, 61, 53];
        let got = order_updates(&updates[3], &rules);
        assert_eq!(got, want);

        let want = vec![61, 29, 13];
        let got = order_updates(&updates[4], &rules);
        assert_eq!(got, want);

        let want = vec![97, 75, 47, 29, 13];
        let got = order_updates(&updates[5], &rules);
        assert_eq!(got, want);

        let want = 61;
        let got = sum_middle_value(&[order_updates(&updates[0], &rules)]);
        assert_eq!(got, want);

        let want = 143;
        let got = sum_middle_value(
            &updates
                .iter()
                .take(3)
                .map(|u| order_updates(u, &rules))
                .collect::<Vec<_>>(),
        );

        assert_eq!(got, want);
    }
}
