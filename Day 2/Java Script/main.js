const fs = require('fs');

///////////////////////////// READ IN DATA //////////////////////////////
function readData(filePath) {
    try {
        const content = fs.readFileSync(filePath, 'utf-8');
        const data = content.trim().split("\n").map(row => 
            row.trim().split(/\s+/).map(Number)
        );
        return data; // Return the array directly
    } catch (error) {
        console.error(`Error reading file ${filePath}:`, error);
        return []; // Return an empty array on error
    }
}

///////////////////////////// PRINT RESULT //////////////////////////////
function printResult(result, i, isDebugMode) {
    if (!isDebugMode) { return; }

    if (result) {
        console.log(`Row ${i}: IS SAFE`);
    } else {
        console.log(`Row ${i}: IS UNSAFE`);
    }
}

///////////////////////////// CHECKS //////////////////////////////
function checkFloor(floor) {
    let isAscending = true;
    let isDescending = true;
    let isValidRange = true;
    
    for (let i = 1; i < floor.length; i++) {
        let diff = Math.abs(floor[i] - floor[i - 1]);

        if (diff < 1 || diff > 3) { 
            isValidRange = false; 
        }

        if (floor[i] < floor[i - 1]) {
            isAscending = false;
        }

        if (floor[i] > floor[i - 1]) {
            isDescending = false;
        }
    }

    return isValidRange && (isAscending || isDescending);
}

function checkFloorWithDampener(floor) {
    if (checkFloor(floor)) {
        return true;
    }

    for (let i = 0; i < floor.length; i++) {
        const tempFloor = floor.slice(0, i).concat(floor.slice(i + 1));
        if (checkFloor(tempFloor)) {
            return true;
        }
    }

    return false;
}

///////////////////////////// MAIN //////////////////////////////
const args = process.argv.slice(2);
const isDebugMode = args[0] === "-d";
const data = readData('InputFile.txt');

if (isDebugMode) {
    console.log("The Data is:", data);
}

/// Part 1
console.log(`PART 1 :`);
let part1Answer = 0;
for (let i = 0; i < data.length; i++) {
    const result = checkFloor(data[i]);
    printResult(result, i, isDebugMode);
    if (result) {
        part1Answer++;
    }
}
console.log("The answer to Part 1 is :", part1Answer);

/// Part 2
console.log(`PART 2 :`);
let part2Answer = 0;
for (let i = 0; i < data.length; i++) {
    const result = checkFloorWithDampener(data[i]);
    printResult(result, i, isDebugMode);
    if (result) {
        part2Answer++;
    }
}
console.log("The answer to Part 2 is :", part2Answer);