A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.

https://projecteuler.net/problem=4

## JavaScript Solution
```
function isPalindrome (string) {
  for (i = 0; i < string.length / 2; i++) {
    if (string[i] !== string[string.length - 1 - i]) {
      return false
    }
  }
  return true
}

function getThreeDigitMultiples () {
  let result = []
  for (i = 100; i <= 999; i++) {
    for (j = 100; j <= 999; j++) {
      result.push(i * j)
    }
  }
  return result
}

const multiples = getThreeDigitMultiples()

let max = 0
multiples.forEach(value => {
  if (isPalindrome(value.toString())) {
    if (value > max) {
      max = value
    }
  }
})

console.log(max)
```
