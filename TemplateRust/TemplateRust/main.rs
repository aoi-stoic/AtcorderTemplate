use std::string::String;

fn read_line2val<T: std::str::FromStr>() -> T{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    s.trim().parse().ok().unwrap()
}

fn read_line2vec<T: std::str::FromStr>() -> Vec<T>{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    s.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect()
}

fn read_line2vecs<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    let mut v2 = Vec::new();
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let v = s.trim().split_whitespace()
            .map(|e| e.parse().ok().unwrap()).collect();
        v2.push(v);
    }
    v2
}

fn main(){
    let a:i32 = read_line2val();
    let bc:Vec<i32> = read_line2vec();
    let s:String = read_line2val();
    println!("{} {}", a + bc[0] + bc[1], s);
}
