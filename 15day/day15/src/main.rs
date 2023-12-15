use std::fs;


fn hasher(s:&str) -> u8 {
    let mut ret:u8 = 0;

    for c in s.chars() {
        ret = ret.wrapping_add(c as u8);
        ret = ret.wrapping_mul(17 as u8);
    }
    
    println!("{} -> hash: {}", s, ret);
    ret
}

fn step_pars(s:&str) -> (String, Option<u8>) {
    match s.split(|e| e == '-' || e == '=').collect::<Vec<_>>().as_slice() {
        [label, typ] => (label.to_string(), typ.parse().ok()),
        _ => panic!()
    }
}

// there are 255 boxes
// there are 1..9 types of lenses
// hash the label up untill either a '-' or a '=' to get the box idx
//      if the label ends with '-' filter all the lenses in the box with that label
//      if the label ends with '=' either insert or replace the lens in the box with the same label
fn main() {
    let input = fs::read_to_string("/home/etonit/advent-of-code2023/15day/input").unwrap();
    let test = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    let steps = input
        .split(",")
        .filter_map(|e| if e.len() != 0
                    {
                        Some(e.chars().filter(|ee| *ee != '\n').collect::<String>())
                    } else { None });


    let mut boxes = vec![Vec::<(String, u8)>::new(); 256];
    for step in steps {

        let (label, lens_typ) = step_pars(&step);
        let box_idx = hasher(&label) as usize;

        if let Some(l_typ) = lens_typ {

            if let Some(to_change) = boxes[box_idx].iter().position(|(l,t)| l == &label) {

                boxes[box_idx][to_change] = (label, l_typ);

            } else {
                boxes[box_idx].push((label, l_typ));
            }

        } else {

            boxes[box_idx] = boxes[box_idx]
                .iter()
                .filter_map(|(l,t)| if &label != l { Some((l.clone(),*t)) } else { None })
                .collect();
        }
    }

// focusing power of len = (1 + box_n) * (lens_idx + 1) * (type_of_len)
    let tot = boxes
        .iter()
        .enumerate()
        .fold(0,|acc,(box_idx,e)| acc +
                      e.iter()
                      .enumerate()
                      .fold(0,|acc,(lens_idx,(_,lens_type))|
                            acc + (1+box_idx) * (1+lens_idx) * (*lens_type as usize)));
    println!("{}", tot);
}

fn includes<T>(slice:&[T], f:impl Fn(&T)->bool) -> bool {
    for e in slice.iter() {
        if f(e) {return true}
    }
    false
}
#[test]
fn stepper() {
    let test = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    let mut steps = test
        .split(",")
        .filter_map(|e| if e.len() != 0
                    {
                        Some(e.chars().filter(|ee| *ee != '\n').collect::<String>())
                    } else { None });

    assert_eq!(step_pars(&steps.next().unwrap()), ("rn".to_owned(), Some(1)));
    assert_eq!(step_pars(&steps.next().unwrap()), ("cm".to_owned(), None));
    assert_eq!(step_pars(&steps.next().unwrap()), ("qp".to_owned(), Some(3)));
}
#[test]
fn hshr(){
    assert_eq!(30, hasher("rn=1"));
}
