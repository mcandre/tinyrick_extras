//! a drinking game

// FB provides a more specific return type for fizzbuzz() than a String,
// as well as using a tad less memory to model this information.
//
// Props to J. M. Archer for the idea!
// https://www.youtube.com/watch?v=sv9fTlU7SCA
#[derive(PartialEq, Eq, Debug)]
pub enum FB {
  Value(i64),
  Fizz,
  Buzz,
  FizzBuzz,
}

/// Play one turn of FizzBuzz
pub fn fizzbuzz(n: i64) -> FB {
  match (n % 3 == 0, n % 5 == 0) {
    (true, true) => FB::FizzBuzz,
    (true, _) => FB::Fizz,
    (_, true) => FB::Buzz,
    _ => FB::Value(n)
  }
}

#[test]
fn smoketest() {
  assert_eq!(fizzbuzz(3), FB::Fizz);
  assert_eq!(fizzbuzz(5), FB::Buzz);
  assert_eq!(fizzbuzz(15), FB::FizzBuzz);
}
