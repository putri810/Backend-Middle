interface Discountable {
    calculateDiscountedPrice(): number;
  }
  
  class Product implements Discountable {
    constructor(
      public name: string,
      public price: number,
      public stock: number,
      public description: string
    ) {}
  
    calculateDiscountedPrice() {
      // Implementasi perhitungan harga setelah diskon
      return this.price * 0.9; // Diskon 10%
    }
  }
  
  // Contoh penggunaan
  const product1 = new Product("Baju", 200000, 50, "Baju keren");
  console.log(product1.calculateDiscountedPrice());
  