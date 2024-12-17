export const calculator = {
  calculate(s) {
    let textEncoder = new TextEncoder();

    let bytes = textEncoder.encode(s);

    let sum = 0;

    for (let i = 0; i < bytes.length; i++) {
      sum += bytes[i];
    }

    return sum % 100 + 1;
  }
};