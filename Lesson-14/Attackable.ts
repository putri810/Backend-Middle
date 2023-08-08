interface Attackable {
    attack(target: Character): void;
  }
  
  class Character implements Attackable {
    constructor(
      public name: string,
      public type: string,
      public level: number,
      public healthPoints: number
    ) {}
  
    attack(target: Character) {
      console.log(`${this.name} menyerang ${target.name}!`);
    }
  }
  
  // Contoh penggunaan
  const player = new Character("Hero", "Warrior", 5, 100);
  const enemy = new Character("Goblin", "Monster", 3, 50);
  
  player.attack(enemy);
  