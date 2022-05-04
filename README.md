# Generational Array

## Why to use it 

Using a generational array presents two main advantages :
- When you remove an element from the array and try to access it, the ABA problem is prevented thanks to generations
- The array will try to allocate the smallest amount of memory by reusing the unused cells in the array.

This module provides a simple interface for working with generational arrays with almost no additionnal runtime complexity compared to a classic vector.

## How to use it

### The types

This module exposes 4 different types

| Name | Description |
| :----: | :----: |
| `GenerationalIndex` | Type representing an index in a generational array. It's made of an index (`usize`) and a generation (`usize`) <br> [See methods](#methods-of-generationalarray) |
| `GenerationalArray` | Generic type representing the generational array |
| `GenerationalArrayResult` | Enum used when getting a reference to an item from the generational array <br> [See variants](#generationalarrayresult) |
| `GenerationalArrayResultMut` | Enum used when getting a mutable reference to an item from the generational array <br> [See variants](#generationalarrayresult) |

### Methods of GenerationalArray

| Method | Use |
| :----: | :----: |
| `new` | Creates new instance of `GenerationalArray` |
| `insert` | Adds an element, takes the value as parameter and returns the index (`GenerationalIndex`) |
| `remove` | Removes an element, takes `index` as parameters and returns a `Result<_, &'static str>` |
| `get` | Get a reference to an element, takes `index` and returns `GenerationalArrayResult` |
| `get_mut` | Get a reference to an element, takes `index` and returns `GenerationalArrayResultMut` |
| `is_empty` | Whether the array is empty or not, returns `bool` |
| `size` | Returns the size (`usize`) of the array |
| `used_size` | Returns the actual number (`usize`) of not empty cells in the array |

### GenerationalArrayResult

| Variant | Meaning |
| :----: | :----: |
| `None` | The current index contains None |
| `OutDated` | The index's generation doesn't match with the current generation |
| `OutOfBounds` | The index is out of bounds |
| `Some` | The value has been found, mutable or not depending whether you called `get` or `get_mut` |

## By Maxooz