interface TaskInterface {
    title: string;
    description: string;
    deadline: Date;
    status: boolean;
}

class Task implements TaskInterface {
    constructor(
      public title: string,
      public description: string,
      public deadline: Date,
      public status: boolean
    ) {}
  }
  
  // Contoh penggunaan
  const task1 = new Task(
    "Mengerjakan Laporan",
    "Selesaikan laporan proyek yang tertunda",
    new Date("2023-08-15"),
    false
  );
  
  console.log(task1);