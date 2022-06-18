fn sort_usernames<T: AsRef<str>>(usernames: &mut Vec<T>) {
    sort_usernames_sol2(usernames);
}

fn sort_usernames_sol1<T: AsRef<str>>(usernames: &mut Vec<T>) {
    usernames.sort_by(|a, b| a.as_ref().to_lowercase().cmp(&b.as_ref().to_lowercase()));
}

fn sort_usernames_sol2<T: AsRef<str>>(usernames: &mut Vec<T>) {
    usernames.sort_by_cached_key(|key| key.as_ref().to_lowercase());
}

fn main() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];

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
