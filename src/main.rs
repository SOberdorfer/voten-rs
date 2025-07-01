use std::any::type_name;

enum Vote {
    In,
    Out,
    Place(u8),
}

struct Name<'a> {
    group: u8,
    round: u8,
    name_ref: &'a str,
    vote: Vote,
}

fn main() {
    let names: Vec<&'static str> = vec![
        "Provide", "Names", "Here", "And", "As", "Many", "As", "You", "Want",
    ];

    let ref_names: Vec<Name> = names
        .iter()
        .map(|name| Name {
            group: 0,
            round: 0,
            name_ref: name,
            vote: Vote::In,
        })
        .collect();

    for round in u8::from(1)..reduce_t(names.len()) {
        let mut group_num = 3;
        let filtered: Vec<(usize, &mut Name)> = ref_names
            .iter_mut()
            .enumerate()
            .filter_map(|(index, name)| match name.vote {
                Vote::Out => None,
                _ => {
                    name.round = round;
                    name.group = &group_num;
                }
            })
            .collect();
        groups.insert(round, create_balanced_groups(names));
        let votes: HashSet<u8, Vote> = vote_groups(groups);
        names = process_votes(votes, &names)
    }
}

fn process_votes(votes: HashSet<u8, Vote>, names: &Vec<&str>) -> Vec<&str> {
    votes
        .iter()
        .map(|key, value| names.iter().find(|name| name == value.third));
    todo!()
}

fn vote_groups(groups: Vec<Vec<&str>>) -> HashSet<u8, Vote> {
    todo!()
}

fn create_balanced_groups(names: Vec<&str>) -> Vec<&Vote> {
    let group_size = 3;
    let amount = names.len();
    let unbalanced: bool = amount % group_size > 0;
    todo!()
}

fn reduce_t<T: TryFrom<usize>>(rx: usize) -> T {
    match rx.try_into() {
        Ok(n) => n,
        Err(_) => {
            panic!("Out of bounds for target {}!", type_name::<T>());
        }
    }
}
