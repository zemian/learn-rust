// https://edabit.com/challenge/ms7pZS4c5TT8KZvt7
fn main() {
	// Test.assertEquals(firstLast("ganesh"), "gh");
	// Test.assertEquals(firstLast("kali"), "ki");
	// Test.assertEquals(firstLast("shiva"), "sa");
	// Test.assertEquals(firstLast("vishnu"), "vu");
	// Test.assertEquals(firstLast("durga"), "da");
	// Test.assertEquals(firstLast("brahma"), "ba");

	println!("{}", firstLast("ganesh".to_string()));
}
fn firstLast(str: str) -> str {
	str[0] + str[-1]
}
// struct Test {}
// impl Test {
// 	fn assertEquals(a: String, b: String) {
// 		if a == b {
// 			println!("PASSED!");
// 		} else {
// 			println!("FAILED!");
// 		}
// 	}
// }