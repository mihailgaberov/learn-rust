fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("s: {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let ss = format!("{}-{}-{}", s1, s2, s3);
    println!("ss: {}", ss);


    let hello = "Здравейте";

    let sss = &hello[0..4];
    println!("sss: {}", sss);

    for c in hello.chars() {
        println!("c {}", c)
    }
}