// By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.
//    3
//   7 4
//  2 4 6
// 8 5 9 3
// That is, 3 + 7 + 4 + 9 = 23.
// Find the maximum total from top to bottom of the triangle below:
//               75
//              95 64
//             17 47 82
//            18 35 87 10
//           20 04 82 47 65
//          19 01 23 75 03 34
//         88 02 77 73 07 63 67
//        99 65 04 28 06 16 70 92
//       41 41 26 56 83 40 80 70 33
//      41 48 72 33 47 32 37 16 94 29
//     53 71 44 65 25 43 91 52 97 51 14
//    70 11 33 28 77 73 17 78 39 68 17 57
//   91 71 52 38 17 14 91 43 58 50 27 29 48
//  63 66 04 68 89 53 67 30 73 16 69 87 40 31
// 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23

// use core::ptr::NonNull;

struct Triangle {
    head: Option<TriangleEntry>,
}

struct TriangleEntry {
    value: u8,
    left_ancestor: Option<TriangleEntry>,
    right_ancestor: Option<TriangleEntry>,
    left_descendent: Option<TriangleEntry>,
    right_descendent: Option<TriangleEntry>,
}

impl Triangle {
    fn new() -> Self {
        Self { head: None }
    }

    fn add_head(&mut self, mut entry: TriangleEntry) {
        self.head = Some(entry);
    }
}

impl TriangleEntry {
    fn new(value: u8) -> Self {
        Self { value: value, left_ancestor: None, right_ancestor: None, left_descendent: None, right_descendent: None }
    }
}

fn main () {
    let test_triangle = Triangle::new();
    let test_entry = TriangleEntry::new(3);
    test_triangle.add_head(test_entry);
    println!("created a triangle i suppose");
}
// Running this gets a compile issue with TriangleEntry needing indirection through Box,
// https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes
// To continue referencing the LinkedList implementation:
// https://doc.rust-lang.org/src/alloc/collections/linked_list.rs.html#423
