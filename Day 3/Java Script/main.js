const fs = require('fs');

function main() {
  const filePath = 'InputFile.txt';
  const data = fs.readFileSync(filePath, 'utf8');

  const instructionsRe = /do\(\)|don't\(\)|mul\(\d+,\d+\)/g;
  const mulExtractRe = /mul\((\d+),(\d+)\)/;

  let isEnabled = true;
  let total = 0;

  const instructions = data.match(instructionsRe) || [];

  for (const instr of instructions) {
    if (instr === 'do()') {
      isEnabled = true;
    } else if (instr === "don't()") {
      isEnabled = false;
    } else if (instr.startsWith('mul(') && isEnabled) {
      const parts = instr.match(mulExtractRe);
      if (parts) {
        const x = parseInt(parts[1], 10);
        const y = parseInt(parts[2], 10);
        total += x * y;
      }
    }
  }

  console.log('Total:', total);
}

main();
