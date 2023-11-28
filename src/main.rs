fn main() {

    let toFind:u64 = 441;
    let mut testNumber: u64 = toFind / 2;
    let mut testNumberSquared = testNumber * testNumber;

    println!("{}", testNumber);
    
    while testNumberSquared != toFind {

        println!("Test number squared {}, to find {}", testNumberSquared, toFind);

        if testNumberSquared > toFind {

            testNumber = testNumber / 2;
            println!("Test bigger");

        } else {

            let testNumSecondHalf = testNumber / 2;

            testNumber = testNumber + testNumSecondHalf;
            println!("Test smaller");

        }

        testNumberSquared = testNumber * testNumber;
        println!("{}", testNumber);

    }

    println!("Final version: \n{}", testNumber);

}