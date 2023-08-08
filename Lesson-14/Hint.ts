interface HintUsable {
    useHint(): void;
    isHintAvailable(): boolean;
  }

  class Hint implements HintUsable {
    constructor(
      public text: string,
      public difficultyLevel: number,
      public points: number,
      public userType: "basic" | "premium"
    ) {}
  
    useHint() {
      console.log(`Petunjuk: ${this.text}`);
    }
  
    isHintAvailable() {
      return this.userType === "premium";
    }
  }
  
  // Contoh penggunaan
  const hint1 = new Hint(
    "Pergi ke ruangan berwarna merah",
    2,
    10,
    "premium"
  );
  
  if (hint1.isHintAvailable()) {
    hint1.useHint();
  } else {
    console.log("Petunjuk tidak tersedia.");
  }
  