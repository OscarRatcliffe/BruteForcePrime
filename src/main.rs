fn main() {
    let toFind:u64 = 64;
    let testNumber: u64 = toFind / 2;
    let testNumberSquared = testNumber * testNumber
    
    while (testNumberSquared != toFind) {

        if (testNumberSquared < toFind) {

            testNumber = testNumber / 2

        } else {

            let testNumSecondHalf = testNumber / 2;

            testNumber = testNumber + testNumSecondHalf;

        }

        println!(testNumber)

    } else {

        println!(testNumber);

    }
}