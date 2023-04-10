pub fn test()
{
    let mut vec = Vec::new();
    for i in 1..=5 { vec.push(i); }

    // same as for v in vec.iter() {}
    for v in &vec { print!("{} ", v); } println!();

    // same as for v in vec.mut_iter() {}
    for v in &mut vec { *v *= 2; }

    // same as for v in vec.into_iter() {}
    for v in vec { print!("{} ", v); } println!();

    // ERROR: vec moved
    //println!("{:?}", vec);
}
