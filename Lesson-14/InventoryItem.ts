interface Categorizable {
    category: string;
  }

  class InventoryItem implements Categorizable {
    constructor(
      public name: string,
      public uniqueCode: string,
      public quantity: number,
      public location: string,
      public category: string
    ) {}
  }
  
  // Contoh penggunaan
  const item1 = new InventoryItem("Laptop", "ABC123", 5, "Rak 1", "Elektronik");
  console.log(item1);
  