mod ref_with_flag;

use ref_with_flag::RefWithFlag;

fn main() {
    let vec = vec![10, 20, 30];
    let mut flagged = RefWithFlag::new(&vec, false);
    assert_eq!(flagged.get_ref()[1], 20);
    flagged.set_flag(true);
    assert_eq!(flagged.get_flag(), true);

    // Unaligned types will not work
    // let unaligned: (bool, [i8; 2]) = (false, [1, 2]);
    // let nope = RefWithFlag::new(&unaligned, false);
}
