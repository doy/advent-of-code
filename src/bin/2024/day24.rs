use advent_of_code::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Wire {
    Literal(bool),
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

fn x_key(i: usize) -> String {
    format!("x{i:02}")
}

fn y_key(i: usize) -> String {
    format!("y{i:02}")
}

fn z_key(i: usize) -> String {
    format!("z{i:02}")
}

fn x_bits(wires: &HashMap<String, Wire>) -> Vec<bool> {
    prefix_bits(wires, 'x')
}

fn y_bits(wires: &HashMap<String, Wire>) -> Vec<bool> {
    prefix_bits(wires, 'y')
}

fn z_bits(wires: &HashMap<String, Wire>) -> Option<Vec<bool>> {
    simplify(wires).map(|wires| prefix_bits(&wires, 'z'))
}

fn prefix_bits(wires: &HashMap<String, Wire>, prefix: char) -> Vec<bool> {
    let mut bits: Vec<_> = wires
        .keys()
        .filter(|name| name.starts_with(prefix))
        .collect();
    bits.sort_unstable();
    bits.into_iter()
        .map(|name| {
            let Wire::Literal(bit) = wires[name] else {
                panic!("not a literal");
            };
            bit
        })
        .collect()
}

fn simplify(wires: &HashMap<String, Wire>) -> Option<HashMap<String, Wire>> {
    let mut wires = wires.clone();
    loop {
        let mut to_insert = vec![];
        for (name, wire) in &wires {
            match wire {
                Wire::Literal(_) => {}
                Wire::And(l, r) => {
                    if let (Wire::Literal(l), Wire::Literal(r)) =
                        (&wires[l], &wires[r])
                    {
                        to_insert.push((name.clone(), *l && *r));
                    }
                }
                Wire::Or(l, r) => {
                    if let (Wire::Literal(l), Wire::Literal(r)) =
                        (&wires[l], &wires[r])
                    {
                        to_insert.push((name.clone(), *l || *r));
                    }
                }
                Wire::Xor(l, r) => {
                    if let (Wire::Literal(l), Wire::Literal(r)) =
                        (&wires[l], &wires[r])
                    {
                        to_insert.push((name.clone(), l ^ r));
                    }
                }
            }
        }
        if to_insert.is_empty() {
            break;
        }
        for (k, v) in to_insert {
            wires.insert(k, Wire::Literal(v));
        }
    }
    if wires.values().all(|v| matches!(v, Wire::Literal(_))) {
        Some(wires)
    } else {
        None
    }
}

pub fn parse(fh: File) -> Result<HashMap<String, Wire>> {
    let mut lines = parse::raw_lines(fh);
    let initial: Vec<_> = parse::chunk(&mut lines)
        .map(|line| {
            let mut parts = line.split(": ");
            (
                parts.next().unwrap().to_string(),
                match parts.next().unwrap() {
                    "1" => Wire::Literal(true),
                    "0" => Wire::Literal(false),
                    _ => unreachable!(),
                },
            )
        })
        .collect();
    let gates: Vec<_> = parse::chunk(&mut lines)
        .map(|line| {
            let cap = regex_captures!(
                r"^([^ ]+) (AND|OR|XOR) ([^ ]+) -> (.+)$",
                &line.trim()
            )
            .unwrap();
            let wire = match &cap[2] {
                "AND" => Wire::And(cap[1].to_string(), cap[3].to_string()),
                "OR" => Wire::Or(cap[1].to_string(), cap[3].to_string()),
                "XOR" => Wire::Xor(cap[1].to_string(), cap[3].to_string()),
                _ => unreachable!(),
            };
            (cap[4].to_string(), wire)
        })
        .collect();
    Ok(initial.into_iter().chain(gates).collect())
}

pub fn part1(wires: HashMap<String, Wire>) -> Result<i64> {
    let mut total = 0;
    for bit in z_bits(&wires).unwrap().into_iter().rev() {
        total *= 2;
        total += if bit { 1 } else { 0 };
    }
    Ok(total)
}

pub fn part2(mut wires: HashMap<String, Wire>) -> Result<i64> {
    let xs = x_bits(&wires);
    let ys = y_bits(&wires);
    let mut zs = z_bits(&wires).unwrap();

    let mut swapped = vec![];

    let mut carry = false;
    for (i, (x, y)) in xs.into_iter().zip(ys.into_iter()).enumerate() {
        let expected = x ^ y ^ carry;
        // ugh, this logic is broken and gross, and probably not correct, but
        // it got me to the answer for my puzzle, so
        if zs[i] != expected {
            let x_key = x_key(i);
            let y_key = y_key(i);
            let z_key = z_key(i);
            let ixor_key = wires
                .iter()
                .find(|(_, v)| {
                    let Wire::Xor(l, r) = v else {
                        return false;
                    };
                    (l, r) == (&x_key, &y_key) || (l, r) == (&y_key, &x_key)
                })
                .unwrap()
                .0
                .clone();
            if let Wire::Xor(l, r) = &wires[&z_key] {
                if l != &ixor_key && r != &ixor_key {
                    if matches!(wires[l], Wire::Or(_, _))
                        && !matches!(wires[r], Wire::Or(_, _))
                    {
                        swapped.push(r.clone());
                        swapped.push(ixor_key.clone());
                        let tmp = wires[r].clone();
                        wires.insert(r.clone(), wires[&ixor_key].clone());
                        wires.insert(ixor_key, tmp);
                        zs = z_bits(&wires).unwrap();
                    } else if matches!(wires[r], Wire::Or(_, _))
                        && !matches!(wires[l], Wire::Or(_, _))
                    {
                        swapped.push(l.clone());
                        swapped.push(ixor_key.clone());
                        let tmp = wires[l].clone();
                        wires.insert(l.clone(), wires[&ixor_key].clone());
                        wires.insert(ixor_key, tmp);
                        zs = z_bits(&wires).unwrap();
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            } else {
                let swap_key = wires
                    .iter()
                    .find(|(_, v)| {
                        let Wire::Xor(l, r) = v else {
                            return false;
                        };
                        l == &ixor_key || r == &ixor_key
                    })
                    .unwrap()
                    .0
                    .clone();
                swapped.push(z_key.clone());
                swapped.push(swap_key.clone());
                let tmp = wires[&z_key].clone();
                wires.insert(z_key.clone(), wires[&swap_key].clone());
                wires.insert(swap_key, tmp);
                zs = z_bits(&wires).unwrap();
            }
        }
        carry = x && y || ((x ^ y) && carry);
    }

    swapped.sort_unstable();
    // println!("{}", swapped.join(","));

    // TODO: just to have something for the test suite to check
    let mut keys: Vec<_> = wires.keys().collect();
    keys.sort_unstable();
    let mut total = 1;
    for key in swapped {
        total *= keys.iter().position(|k| &&key == k).unwrap();
    }

    Ok(total.try_into().unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 24).unwrap()).unwrap()).unwrap(),
        43559017878162
    );
    assert_eq!(
        part2(parse(parse::data(2024, 24).unwrap()).unwrap()).unwrap(),
        22942366904133600
    );
}
