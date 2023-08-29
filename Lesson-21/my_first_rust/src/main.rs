use std::io;

fn main() {
    println!("Kalkulator Sederhana");
    loop {
        println!("Pilih operasi:");
        println!("1. Penjumlahan");
        println!("2. Pengurangan");
        println!("3. Perkalian");
        println!("4. Pembagian");
        println!("5. Keluar");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Gagal membaca input");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Pilihan tidak valid, silakan pilih nomor 1-5.");
                continue;
            }
        };

        if choice == 5 {
            println!("Terima kasih telah menggunakan kalkulator ini.");
            break;
        }

        println!("Masukkan dua angka:");

        let mut num1 = String::new();
        let mut num2 = String::new();

        io::stdin()
            .read_line(&mut num1)
            .expect("Gagal membaca input");
        io::stdin()
            .read_line(&mut num2)
            .expect("Gagal membaca input");

        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input pertama tidak valid.");
                continue;
            }
        };

        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input kedua tidak valid.");
                continue;
            }
        };

        match choice {
            1 => println!("Hasil: {}", num1 + num2),
            2 => println!("Hasil: {}", num1 - num2),
            3 => println!("Hasil: {}", num1 * num2),
            4 => {
                if num2 != 0.0 {
                    println!("Hasil: {}", num1 / num2);
                } else {
                    println!("Tidak bisa membagi dengan nol.");
                }
            }
            _ => println!("Pilihan tidak valid, silakan pilih nomor 1-5."),
        }
    }
}
