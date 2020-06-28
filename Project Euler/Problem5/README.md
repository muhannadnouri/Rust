2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

https://projecteuler.net/problem=5

## JavaScript Solution
```
// Brute Force Solution
const isDivisibleByRange = (number) => {
  for (i = 1; i <= 20; i++) {
    if (number % i !== 0) { return false }
  }
  return true
}


let smallestDivisibleNumber = 20
while (true) {
  if (isDivisibleByRange(smallestDivisibleNumber)) {
    break
  } else {
    smallestDivisibleNumber++
  }
}

console.log(smallestDivisibleNumber)
```
