use std::collections::HashSet;

struct Voting {
    first: Vec<i16>
    second: Vec<i16>
    third: Vec<i16>
}

fn main() {
    let names = vec![
        "Everett", "Elio", "Fergus", "Gilbert", "Jarvis", "Jed", "Magnus", "Otis", "Parker",
    ];
    let groups: Vec<Vec<&str>> = create_balanced_groups(names);
    let votes: HashSet<i16, Voting> = vote_groups(groups)
    verify_votes(votes)
}

fn vote_groups(groups: Vec<Vec<&str>>) -> HashSet<i16, Voting> {
    todo!()
}

fn create_balanced_groups(names: Vec<&str>) -> Vec<Vec<&str>> {
    let group_size = 3
    let amount = names.len();
    let unbalanced: bool = amount % group_size > 0;
    todo!()
}
