fn sort_usernames<T: AsRef<str> + std::clone::Clone>(usernames: &mut Vec<T>) {
    for _ in 0..usernames.len() {
        let mut i = 0;
        while i < usernames.len() - 1 {
            let name = usernames[i].as_ref().to_lowercase();
            let name_next: String = usernames[i + 1].as_ref().to_lowercase();
            let temp = if name.len() < name_next.len() {
                name.len()
            } else {
                name_next.len()
            };
            for j in 0..temp {
                if name.chars().nth(j) == name_next.chars().nth(j) {
                    continue;
                } else {
                    if name.chars().nth(j) > name_next.chars().nth(j) {
                        usernames.swap(i, i + 1)
                    }
                    break;
                }
            }
            i += 1;
        }
    }
}

fn main() {
    let mut users = vec!["@Todd", "$Amy", "mike99", "Jennifer", "alison"];

    println!("unsorted: {:?}", &users);
    sort_usernames(&mut users);
    println!("sorted:   {:?}", &users);
}

#[test]
fn five_users() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut users);

    assert_eq!(users, sorted);
}
