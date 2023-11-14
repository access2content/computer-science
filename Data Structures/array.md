- [Introduction](#introduction)
- [Properties of an Array](#properties-of-an-array)
- [Operations on an Array](#operations-on-an-array)
	- [Adding an element to an Array](#adding-an-element-to-an-array)
	- [Removing an element from the array](#removing-an-element-from-the-array)
	- [Reading an element from the array](#reading-an-element-from-the-array)
	- [Updating an array element](#updating-an-array-element)
- [Learning](#learning)
	- [Arrays are fast at](#arrays-are-fast-at)
	- [Arrays are slow at](#arrays-are-slow-at)
	- [When to use an array](#when-to-use-an-array)
- [Array Algorithms](#array-algorithms)
- [Questions](#questions)

# Introduction
An array data structure is a collection of a particular data type stored
at contiguous memory locations. Since most of the tasks that one has to
perform will be done on similar type of data, having them close to each
other is a great way to save computation time in finding and updating
data.

Think of an array as a row in an Excel Sheet. You can easily navigate to
a neighboring element by simply going left or right. In terms of data
storage, you can access an adjacent element by going to the next or
previous memory address.

Now, the question is, how many ELEMENTS will an array store? An array
can store as many elements in contiguous locations as you wish. An array
SIZE can be pre-determined or it can be dynamic. HOW you use an array is
up to the use case. But the concept is the same - \"An array is a
collection of a particular data type stored in contiguous memory
location\".

Now that we know WHAT an array is, let us look at some PROPERTIES of an
array. Let\'s assume we take an array of 10 number data type.

# Properties of an Array
An array should have the same data type for each of its elements.

An array will be stored in a continuous location. So, if we want to find
an element, we can get the memory address by simply multiplying the size
of the data type with the element index.

# Operations on an Array
The most obvious operation we would want to do is to ADD an element to
the array, followed by DELETE an element from the array. Next, we would
like to READ an element in the array and then UPDATE an element in the
array.

## Adding an element to an Array
If you look at an array, they are always going to be in a continuous
memory location. Therefore, there are two ways of inserting elements in
the array:

-   Insert an element at the end
-   Insert an element anywhere in the array

The difference between these two operations is that if you want to add
an element to the end, you won\'t have to SHIFT the existing array.
Whereas, if an element is to be inserted anywhere except the end, we
would have to shift all the elements till the end of the array. Let\'s
understand with an example.

Let\'s take an array: 1, 2, 4, 5 Let\'s assume the element 1 is stored
in the memory at the address: x0123 and each element consumes 1 byte of
memory. So, the array along with the memory address would look like:

``` example
1       2       4       5
x0123   x0124   x0125   x0126
```

If we add the element 6 at the end, the array will become:

``` example
1       2       4       5       6
x0123   x0124   x0125   x0126   x0127
```

However, if want to add 3 at the memory address x0125, we would first
have to shift all the elements to the right as follows:

``` example
1       2               4       5       6
x0123   x0124   x0125   x0126   x0127   x0128
```

Now that the memory address is empty, we can insert the new element.
Hence, adding an element to an array at the last index is fast, but
adding it anywhere else is slow.
## Removing an element from the array
Removing an array element follows the same pattern as adding an element
in the array. An element from an array can be removed from any position
in the array. If however, we are removing an element from an array not
at the last index, we have to shift all the elements after the deleted
element to the left. Let\'s understand with the same example:

``` example
1       2       4       5       6
x0123   x0124   x0125   x0126   x0127
```

If we wish to delete 4 from the array, we would have to shift 5 and 6 to
the left to make it an array.

``` example
1       2       5       6
x0123   x0124   x0125   x0126
```
## Reading an element from the array
Reading an element from an array is where the power of an array is
harnessed. Essentially, you can think of an array as a data structure
which makes READING easier as compared to EDITING.

When a data is to be read from an array, it essentially means getting
the VALUE at an INDEX in the array. Let us understand both these terms
before proceeding further.

An INDEX is the nth memory location of an array. Let\'s look at the
previous example along with the index, value, and memory of each
element. Let\'s call this array \"myArray\" which starts at the memory
location x0123

myArray -\> x0123

``` example
Value ->    1       2       4       5       6
Memory ->   x0123   x0124   x0125   x0126   x0127
Index ->    0       1       2       3       4
```

Let\'s say we want to read the 3rd element in myArray. The computer
would simply take the first memory address - x0123 and skip 2 elements
to reach the 3rd element. Therefore, to reach the 3rd element (x0125),
the computer would first go to x0123, then skip it to reach x0124, then
skip it to reach x0125 which is the 3rd element in myArray.

Similarly, to reach the 5th element in the array (x0127), the computer
would go to the base of the array x0123 and skip 4 elements to reach the
5th elements.

Therefore, in general, to reach the nth element in an array, the
computer would first go to the base of the array and skip n-1 elements.

Now let\'s leverage this understanding of the array with the properties
of an array. We know that the array is stored in a continuous memory
location and each element consumes the same amount of space in memory.
Therefore, instead of skipping memory locations, we can directly
calculate the memory address of an element.

Let\'s try to reach the 3rd element of an array. We know the base
address of the array is x0123. We also know that each element in the
array consumes 1 byte of memory space. We also know that to reach the
3rd element, we need to skip 2 elements. Now, we can easily calculate
the total space occupied by 2 elements by simply multiplying the no. of
elements to skip with memory size of each element.

Hence, the formula (size of each element \* no. of elements to skip)
would give us the no. of bytes to skip to reach our destination. So, (1
\* 2) = 2. We can simply add this to our base address to reach the
destination. So, x0123 + 2 = x0125. This is the address of the 3rd
element.

Try doing it for reaching the 5th element of the array.

So, in general, if we want to reach the nth element of an array, we can
simply calculate (size of an element \* (n-1)) to jump to the nth
location in the array. This nth location is known as the INDEX of the
array.

An observation is that arrays are designed to be used for READING via
the INDEX of the array. It is not a great data structure for finding the
VALUE at an INDEX, but it is very powerful when it comes to using an
array with index.

Let\'s take reading with array VALUE in the same <example:->

myArray -\> x0123

``` example
Value ->    1       2       4       5       6
Memory ->   x0123   x0124   x0125   x0126   x0127
Index ->    0       1       2       3       4
```

If we want to know at which INDEX the value 5 is, there is no simple way
for the computer to find it. We would HAVE TO go through each element in
each index till we find the element 5 (unless they are stored via some
logic).

## Updating an array element

Just like reading an array element, updating an array element is a very
fast operation. In the following example, if we want to update the value
4 to 3, we just have to go to that index and change the value to 3.
Let\'s assume we are at the index at which we want the update the value.

myArray -\> x0123

``` example
Value ->    1       2       4       5       6
Memory ->   x0123   x0124   x0125   x0126   x0127
Index ->    0       1       2       3       4
```

We can just change the value from 4 to 3. The only thing that will take
time is to reach the index at which we want to update. Therefore, if we
know the index that we want to update, we can do it quickly. But, if we
only know the value that we want to change, we will first have to find
the value in the array by traversing through it and then update the
value.

# Learning
Arrays store data in contiguous memory locations Arrays store the same data type in all locations Arrays are fast at reading data from an Index Arrays are slow at reading data by value Arrays are slow when we have to add data in the middle Arrays are fast when we have to add data in the end Arrays are slow when we have to remove data from the middle Arrays
are fast when we have to remove data from the end Arrays are fast at updating data at an index
## Arrays are fast at
- Reading data from an Index 
- Updating data at an Index 
- Adding data at the end  
- Removing data at the end
## Arrays are slow at
- Reading data by value 
- Adding data in the middle  
- Removing data from the middle
## When to use an array
- When we won\'t add/remove data frequently 
- When the data to be added or removed is mostly at the end index  
- When we can derive a relation of the array value with the index to allow faster processing  
- When we want to store same data type for quick access

# Array Algorithms
- Sorting
- Searching
- Binary Search
- Two Pointers
- Subarray
- Prefix sum and carry forward
- Subset
- Subsequence

# Questions
[Geeks for Geeks](https://www.geeksforgeeks.org/top-50-array-coding-problems-for-interviews/)

- Find the max and min element in an array
- Reverse an array
- Find the kth largest and the kth smallest element in an array
- Given an array arr[] of size N-1 with integers in the range of `[1, N]`, find the missing number from the first N integers. ([Solution](https://www.geeksforgeeks.org/find-the-missing-number/))