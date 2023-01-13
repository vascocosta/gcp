# Puzzles

[P1 - Mean Median Mode](#p1-mean-median-mode)

[P2 - Batch Operations](#p2-batch-operations)

## P1 Mean Median Mode

### Description

Write a program that takes a list of **integers** as input and outputs the mean, median, and mode of the list.

Remember that the mean is the average value, the median is the middle number in a sorted list of numbers **(or the average of the two middle numbers for lists with an even number of items)**, the mode is the most frequent number **or numbers** in a list of numbers.

Use a maximum of **two decimal places** for **non-integer** mean and median values. Sometimes it doesn't make sense to calculate some of these values, in which case you should display "None" instead. The mode can be a list with a single number **or a list of multiple numbers**.

The program should be able to handle lists of any length, including **empty lists**.

#### Examples

```
Input: [1, 2, 3]
Output: Mean: 2, Median: 2, Mode: None
```
```
Input: [4, 11, 2, 6, 10, 20]
Output: Mean: 8.83, Median: 8, Mode: None
```
```
Input: [3, 6, 2, 6, 1, 6, 1]
Output: Mean: 3.57, Median: 3, Mode: [6]
```
```
Input: [1, 1, 2, 2, 2, 3, 3, 3, 4]
Output: Mean: 2.33, Median: 2, Mode: [2, 3]
```
```
Input: [1]
Output: Mean: 1, Median: 1, Mode: None
```
```
Input: []
Output: Mean: None, Median: None, Mode: None
```

#### Notes

1. You can provide the input hardcoded in your program, instead of getting it from the user or a file.
2. Try to avoid library functions/methods that calculate the mean, median or mode.
3. The modulo operator (usually %) is helpful to determine whether a list has an even/odd number of items.


## P2 Batch Operations

### Description

Write a program that takes a list of **strings** corresponding to simple two-argument operations comprising addition, subtraction, multiplication and division, with infix operator as input and outputs a list of **strings** with the results.

Use a maximum of **two decimal places** for **non-integer** results. In ordinary arithmetic dividing by zero has no meaning, at least not without considering the concept of limit of a function. In computing, dividing by zero often results in an error, thus, your program should be aware of any operation where the denominator is zero and consider such operations as an error as well. The result is not defined in these cases, so your program should display the "ND" string as the result.

The program should be able to handle lists of any length, including **empty lists**. It should also be resilient to inputs where the operation strings present a variable number of white space charaters including the lack of any white space, calculating the correct results without crashing.

#### Examples

```
Input: ["1 + 1", "10 - 7", "4 * 25", "7 / 3"]
Output: ["2", "3", "100", "2.33"]
```
```
Input: ["17.5 + 2.50", "2 - 1.5", "3 * 4.155", "20 / 0"]
Output: ["20", "0.50", "12.47", "ND"]
```
```
Input: ["5+7 ", "45 -    5", "  10  *10", "0   /   0"]
Output: ["12", "40", "100", "ND"]
```
```
Input: ["1-1"]
Output: ["0"]
```
```
Input: []
Output: []
```

#### Notes

1. You can provide the input hardcoded in your program, instead of getting it from the user or a file.
2. While not necessary, consider defining a type to represent operations.