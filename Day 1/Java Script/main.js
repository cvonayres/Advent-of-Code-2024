const fs = require('fs');

///////////////////////////// READ IN DATA //////////////////////////////
function readColumns(filePath) {
    try {
        const data = fs.readFileSync(filePath, 'utf-8');
        const leftColumn = [];
        const rightColumn = [];

        data.split('\n').forEach(line => {
            const [leftValue, rightValue] = line.split('   ').map(value => parseInt(value, 10));
            if (Number.isInteger(leftValue) && Number.isInteger(rightValue)) {
                leftColumn.push(leftValue);
                rightColumn.push(rightValue);
            }
        });

        return { leftColumn, rightColumn };
    } catch (error) {
        console.error(`Error reading file ${filePath}:`, error);
        return { leftColumn: [], rightColumn: [] };
    }
}

const { leftColumn, rightColumn } = readColumns('InputFile.txt');
const sortedLeftColumn = [...leftColumn].sort((a, b) => a - b);
const sortedRightColumn = [...rightColumn].sort((a, b) => a - b);

//////////////////////////////// PART 1 ////////////////////////////////
let part1Answer = 0;

for (let i = 0; i < sortedLeftColumn.length; i++) {
    const diff = Math.abs(sortedLeftColumn[i] - sortedRightColumn[i]);
    part1Answer += diff;
}
console.log("Part 1 Answer is:", part1Answer);

//////////////////////////////// PART 2 ////////////////////////////////
let part2Answer = 0;

for (let i = 0; i < leftColumn.length; i++) {
    let multiple = 0;
    for (let j = 0; j < rightColumn.length; j++) {
        if (leftColumn[i] === rightColumn[j]) {
            multiple++;
        }
    }
    part2Answer += leftColumn[i] * multiple;
}

console.log("Part 2 Answer is:", part2Answer);