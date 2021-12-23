// #############
// #abcdefghijk#
// ###l#m#n#o###
//   #p#q#r#s#
//   #########
static NEIGHBORS1: &[&[usize]] = &[
    &[1],
    &[0, 2],
    &[1, 3, 11],
    &[2, 4],
    &[3, 5, 12],
    &[4, 6],
    &[5, 7, 13],
    &[6, 8],
    &[7, 9, 14],
    &[8, 10],
    &[9],
    &[2, 15],
    &[4, 16],
    &[6, 17],
    &[8, 18],
    &[11],
    &[12],
    &[13],
    &[14],
];

// #############
// #abcdefghijk#
// ###l#m#n#o###
//   #p#q#r#s#
//   #t#u#v#w#
//   #x#y#z#!#
//   #########
static NEIGHBORS2: &[&[usize]] = &[
    &[1],
    &[0, 2],
    &[1, 3, 11],
    &[2, 4],
    &[3, 5, 12],
    &[4, 6],
    &[5, 7, 13],
    &[6, 8],
    &[7, 9, 14],
    &[8, 10],
    &[9],
    &[2, 15],
    &[4, 16],
    &[6, 17],
    &[8, 18],
    &[11, 19],
    &[12, 20],
    &[13, 21],
    &[14, 22],
    &[15, 23],
    &[16, 24],
    &[17, 25],
    &[18, 26],
    &[19],
    &[20],
    &[21],
    &[22],
];

fn path_rec(
    neighbors: &[&[usize]],
    from: usize,
    to: usize,
    path: &mut Vec<usize>,
) -> bool {
    if from == to {
        return true;
    }
    for neighbor in neighbors[from] {
        if path.contains(neighbor) {
            continue;
        }
        path.push(*neighbor);
        if path_rec(neighbors, *neighbor, to, path) {
            return true;
        }
        path.pop();
    }
    false
}

fn path(neighbors: &[&[usize]], from: usize, to: usize) -> Vec<usize> {
    let mut path = vec![from];
    if !path_rec(neighbors, from, to, &mut path) {
        panic!("no path found from {} to {}", from, to);
    }
    path.remove(0);
    path
}

fn main() {
    println!("static CONNECTIVITY1: &[&[&[usize]]] = &[");
    for from in 0..19 {
        println!("    &[");
        for to in 0..19 {
            let path = path(NEIGHBORS1, from, to);
            let path_strs: Vec<_> =
                path.iter().map(|i| i.to_string()).collect();
            println!("        &[{}],", path_strs.join(", "));
        }
        println!("    ],");
    }
    println!("];");
    println!();
    println!("static CONNECTIVITY2: &[&[&[usize]]] = &[");
    for from in 0..27 {
        println!("    &[");
        for to in 0..27 {
            let path = path(NEIGHBORS2, from, to);
            let path_strs: Vec<_> =
                path.iter().map(|i| i.to_string()).collect();
            println!("        &[{}],", path_strs.join(", "));
        }
        println!("    ],");
    }
    println!("];");
}
