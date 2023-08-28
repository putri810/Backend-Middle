// Mendefinisikan structs bernama Employee dengan empat properti: name, position, salary, dan department
struct Employee {
    name: String,
    position: String,
    salary: u32,
    department: String,
}

// Menerapkan methods pada structs Employee
impl Employee {
    // Method untuk menampilkan informasi pegawai
    fn display(&self) {
        println!("Name: {}", self.name);
        println!("Position: {}", self.position);
        println!("Salary: {}", self.salary);
        println!("Department: {}", self.department);
    }

    // Method untuk memberikan bonus gaji kepada pegawai
    fn give_bonus(&mut self, amount: u32) {
        self.salary += amount;
        println!("{} received a bonus of {}", self.name, amount);
    }
}

fn main() {
    // Membuat instance structs Employee
    let mut emp1 = Employee {
        name: String::from("Alice"),
        position: String::from("Manager"),
        salary: 5000,
        department: String::from("Marketing"),
    };

    // Memanggil method display pada emp1
    emp1.display();
    /*
    Name: Alice
    Position: Manager
    Salary: 5000
    Department: Marketing
    */

    // Memanggil method give_bonus pada emp1 dengan argumen 1000
    emp1.give_bonus(1000); // Alice received a bonus of 1000

    // Memanggil method display pada emp1 lagi untuk melihat perubahan gaji
    emp1.display();
    /*
    Name: Alice
    Position: Manager
    Salary: 6000
    Department: Marketing
    */
}
