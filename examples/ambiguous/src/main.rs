use munchies::run::run;
// use munchies::traits::Parser;
use munchies::{
    combinators::{many, then},
    helper::char,
};

fn main() {
    let a = char::char(|c| c == 'a');
    // let b = char::char(|c| c == 'b');
    let many_as = many::many(a);
    let ambiguous_p = then::then(many_as, a);
    // let ambiguous_p = then::then(ambiguous_p, b);

    let result = run(ambiguous_p, "aaaabc");
    println!("{:?}", result);
}
