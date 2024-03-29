function colorResistance(color: string): number {
  switch (color) {
    case "black":
      return 0;
    case "brown":
      return 1
    case "red":
      return 2
    case "orange": 
      return 3;
    case "yellow": 
      return 4;
    case "green": 
      return 5;
    case "blue": 
      return 6;
    case "violet": 
      return 7;
    case "grey": 
      return 8;
    case "white": 
      return 9;
    default:
      throw new Error("Invalid Color");
  }
}

export function decodedResistorValue(colors: string[]): string {
  let resistance: string | number = colorResistance(colors[0]).toString() + colorResistance(colors[1]).toString();
  let ohmMultiplier = colorResistance(colors[2]);

  if (ohmMultiplier > 0) {
    while (ohmMultiplier > 0) {
      resistance += "0";
      ohmMultiplier = ohmMultiplier - 1;
    }
  }

  resistance = Number(resistance);

  return `${resistance > 1000 ? resistance / 1000 : resistance} ${resistance > 1000 ? 'kiloohms': 'ohms'}`;
}
