# Puzzles

[P1 - Mean Median Mode](##mean-median-mode)

## Mean Median Mode

### Description

Write a program that takes a list of **integers** as input and outputs the mean, median, and mode of the list. Remember that the mean is the average value, the median is the middle number in a sorted list of numbers **(or the average of the two middle numbers for lists with an even number of numbers)**, the mode is the most frequent number **or numbers** in a list of numbers. Use a maximum of **two decimal places** for **non-integer** mean and median values. Sometimes it doesn't make sense to calculate some of these values, in which case you should display "None" instead. The mode can be a list with a single number **or a list of multiple numbers**. The program should be able to handle lists of any length, including **empty lists**.

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