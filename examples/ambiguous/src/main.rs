use munchies::{
    combinators::{many, then},
    helper::char,
    run,
};

fn main() {
    let a = char(|c| c == 'a');
    // let b = char::char(|c| c == 'b');
    let many_as = many(a);
    let ambiguous_p = then(many_as, a);
    // let ambiguous_p = then::then(ambiguous_p, b);

    let result = run(ambiguous_p, "aaaabc");
    println!("{:?}", result);
}
