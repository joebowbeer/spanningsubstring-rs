use hashlink::LinkedHashMap;
use std::collections::HashSet;
use std::env::args;
use std::hash::Hash;

fn main() {
    let goal = args().nth(1).expect("required: goal");
    let pat = args().nth(2).expect("required: pat");
    let (pos, len) = span(pat.chars(), HashSet::from_iter(goal.chars()));
    println!("goal = {goal}");
    println!("pattern = {pat}");
    println!("Shortest span = [{}]", &pat[pos..(pos + len)]);
}

/// Given a sequence of elements and a set of elements, finds the shortest
/// subsequence containing all the elements in the set.
/// <p>
/// For example, "CYBXA" is the shortest substring of "ZCYBXAW" that
/// contains the characters of "ABC".
fn span<T: Eq + Hash, U: IntoIterator<Item = T>>(pat: U, goal: HashSet<T>) -> (usize, usize) {
    let goal_len = goal.len();
    let mut pos = 0;
    let mut len = 0;
    let mut map = LinkedHashMap::<T, usize>::new();
    let pairs = pat.into_iter().enumerate().filter(|p| goal.contains(&p.1));
    for p in pairs {
        // append next T->index to linked map
        map.insert(p.1, p.0);
        // if we have a shorter span, record it
        let head = *map.front().unwrap().1;
        let tail = *map.back().unwrap().1;
        let newlen = tail - head + 1;
        if map.len() == goal_len && (len == 0 || newlen < len) {
            pos = head;
            len = newlen;
            // return early if optimal solution found
            if len == goal_len {
                return (pos, len);
            }
        }
    }
    (pos, len)
}

#[test]
fn span_test() {
    assert_eq!(
        (0, 0),
        span("".chars(), HashSet::from_iter("A".chars())),
        "empty pat"
    );
    assert_eq!(
        (0, 0),
        span("A".chars(), HashSet::from_iter("".chars())),
        "empty goal"
    );
    assert_eq!(
        (0, 5),
        span("ABCDEFG".chars(), HashSet::from_iter("BEAD".chars())),
        "ABCDE"
    );
    assert_eq!(
        (1, 5),
        span("ZCYBXAW".chars(), HashSet::from_iter("ABC".chars())),
        "CYBXAW"
    );
    assert_eq!(
        (2, 3),
        span("CAABC".chars(), HashSet::from_iter("ABC".chars())),
        "ABC"
    );
    assert_eq!(
        (0, 6),
        span([1, 2, 3, 4, 5, 6], HashSet::from([6, 1])),
        "numeric"
    );
}
