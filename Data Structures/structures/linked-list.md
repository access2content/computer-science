- [Introduction](#introduction)
- [Advantages of Linked list Data Structures](#advantages-of-linked-list-data-structures)
- [Disadvantages of Linked list Data Structures](#disadvantages-of-linked-list-data-structures)

# Introduction

A linked data structure is a structure which consists of a set of
records linked together by references of memory location instead of
being stored in contiguous memory locations. Each data record stores the
memory address of another data record, in essence linking the data
records.

Even if all the individual data records are stored in an array or at
continuous location, they will be a linked data structure as no
mathematical computation is done to ascertain the location of another
data record.

Some linked Data structures include:

-   Linked Lists
-   Search Trees
-   Expression Trees

# Advantages of Linked list Data Structures

Linked lists are great at making use of available memory space as they
do not require contiguous memory for storage.

They do not require one to instantiate a fixed memory at the beginning
which could be a potential waste of memory storage.

A linked data structure will only use the required amount of memory.

If required, individual Nodes of the DS can be moved without causing
loss of data which is not possible in the case of array data structure.

# Disadvantages of Linked list Data Structures

Accessing random elements of a Linked DS is slow as we cannot perform a
mathematical calculation to reach a node. We will have to traverse each
node form the base location to reach the desired node.

In some cases, the references tend to occupy more space than the actual
data structure.

There is an overhead of memory allocation for each new node creation
every time we add a new data point.
