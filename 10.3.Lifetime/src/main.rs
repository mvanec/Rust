fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result);
    }

    let v = vec![5, 4, 3, 2, 1];
    let n = &v[0];
    let g = find_nth(&v, 4);
    println!("n is {n} and g is {g}");

    let v = vec![5, 4, 3, 2, 1];
    let g = find_nth2(&v, 2);
    println!("g is {g} and vec is {:?}", &v);

    let tr = &mut TestResult {
        scores: vec![5, 10, 15, 20],
        curve: Some(10),
    };
    tr.apply_curve();
    println!("{:?}", tr.scores);

}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// Returns the n-th largest element in a slice
fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    let mut elems = elems.to_vec();
    elems.sort();
    let t = &elems[n];
    return t.clone();
}

fn find_nth2<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    let mut elem_refs: Vec<&T> = elems.iter().collect();
    elem_refs.sort();
    let t = elem_refs[n];
    return t.clone();
}

struct TestResult {
    /// Student's scores on a test
    scores: Vec<usize>,

    /// A possible value to curve all scores
    curve: Option<usize>
}
impl TestResult {
    // pub fn get_curve(&self) -> &Option<usize> {
    //     &self.curve
    // }

    /// If there is a curve, then increments all
    /// scores by the curve
    pub fn apply_curve(&mut self) {
        if let Some(curve) = self.curve {
            for score in self.scores.iter_mut() {
                *score += curve;
            }
        }
    }
}
