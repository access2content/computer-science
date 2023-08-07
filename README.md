- [What is a Data Type?](#what-is-a-data-type)
- [Data Storage](#data-storage)
- [What is a Data Structure?](#what-is-a-data-structure)
- [Implementation of Data Structures](#implementation-of-data-structures)
- [Abstract Data Types (ADT)](#abstract-data-types-adt)
- [Operations on Abstract Data Types](#operations-on-abstract-data-types)
# What is a Data Type?
First, let\'s be sure to know what a [Data Type](Data%20Type.md) in order to avoid confusion. Data types can be primitive and non-primitive. Primitive data types are data types with their own set of properties which cannot be further broken down. Non-Primitive data types are any combination of primitive data types with its own properties and operations.
# Data Storage
Now, it is easy to store a single data in memory in the computer.
However, it would be troublesome to store large amounts of data in the
computer without there being a relation between the data elements.
Let\'s assume we have an Excel sheet as our storage device. How would
you store phone numbers in the sheet?

You can assign either a Column or a Row for Phone numbers and use only
that to store phone numbers. This way, you can quickly find phone
numbers in the sheet. Secondly, you might choose to store phone numbers
and random locations on the sheet. This way, you will be able to store
the data, but you might have difficulty find the phone numbers in the
future as it will be mixed along with other data as well.

Whatever the case, you will soon realize that there are only 3 ways of
storing your data - Sequentially, Logically or Randomly. Randomly
storing data is self-explanatory. However, we can store data not only
sequentially in our sheet, we can store data via some logic as well. For
example, we can store phone numbers in cells at odd locations only. This
way, our data is still easy to find and relate. So, we can store data in
the memory in one of 3 ways:

-   Sequential
-   Logical
-   Random
# What is a Data Structure?
Therefore, Data Structure refers to the system of storing data types in
memory. The way you store, access, modify data provides in memory gives
you a \"structure\" to work with data more efficiently for different use
cases.

# Implementation of Data Structures
We can implement any data structure by placing data in memory either
sequentially, logically, or randomly. The Memory Address of a data type
can be stored in a variable so that the computer can understand WHERE to
find the data. This variable that stores the address of another variable
is known as a Pointer.

Therefore, if we want to find the memory address of the next element, we
can either perform mathematical calculations to find it, or store the
address directly in a pointer. These are the two approaches that is used
for implementing Data Structures.

The mathematical implementation of storage is referred to as Array Data
Structure, and pointer implementation is referred to as Linked Data
Structure.

- [Array Data Structure](structures/array.md)
- [Linked List Data Structure](structures/linked-list.md)
# Abstract Data Types (ADT)
An abstract data type is a mathematical/conceptual model of a data
structure that specifies the data type, the operations, and the
parameters of the operations possible on the data type. The concept does
not care about the implementation of the ADT. It is up to you to choose
the implementation method that suits your requirements best.

Some of the most commonly use ADTs are:
-   [Stack](adt/stack.md)
-   Queue
-   Tree
-   Hashmap
-   Tuple
-   Graph
# Operations on Abstract Data Types
Since data is going to be stored on a storage device, we would want to
perform the following operations on the data. These are the most basic
operations that one can expect in almost all data types. Of course each
data structure is different with different use case. However, these are
the most primitive operations that one can expect:

-   Add elements to the structure
-   Get the value of an element
-   Update the value of an element
-   Delete an element