use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Bir sayı tahmin et!");
    let gizli_sayı = rand::random_range(1..=100);

    loop {
        println!("Sayını gir:");
        let mut tahmin = String::new();
        io::stdin().read_line(&mut tahmin).expect("hatalı");
        let tahmin: i32 = match tahmin.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("tahminin {tahmin}");

        match tahmin.cmp(&gizli_sayı) {
            Ordering::Less => println!("çok küçük"),
            Ordering::Greater => println!("çok büyük"),
            Ordering::Equal => {
                println!("doğru tahmin");
                break;
            }
        }
    }
}
