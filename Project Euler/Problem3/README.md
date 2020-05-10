The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?

https://projecteuler.net/problem=3

## JavaScript Solution
```
const isPrime = (value) => {
    for(i = 2; i < value; i++) {
        if(value % i === 0) {
            return false;
        }
    }
    return value > 1;
}

const bigNumber = 600851475143
const primeFactors = []

for (let i = 1; i <= bigNumber; i++) {
  if (bigNumber % i === 0 && isPrime(i)){
    primeFactors.push(i);
  }
}

console.log(Math.max(...primeFactors))
```

#### Resources
- [Vectors](https://learning-rust.github.io/docs/b1.vectors.html)
- [Vectors Multiple Iterations](https://old.reddit.com/r/rust/comments/3s2obn/multiple_iterations_over_same_vector/)
