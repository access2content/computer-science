# Introduction

Computers need to store data. The way data is stored is very important
as different types of data are stored differently and treat differently
by the computer.

A \"number\" for example has properties of Addition, Subtraction,
Multiplication, and so on. For example, we can multiply two numbers
together - 5 \* 6 = 30. Numbers contain the characters from 0 to 9.

A \"word\" on the other hand has different type of properties. A word
cannot be \"multiplied\" with another word. However, characters and
words can be combined to form new words. For example. the word \"good\"
has a particular meaning. We can add \"not\" before the word which
changes its meaning. Words contain characters from a-z and A-Z.

A \"date\" has different properties when compared to number or word. A
unit of time, let\'s say minute, or second can only have increments of
60, whereas hours can have a upper limit of 24. Therefore, we cannot ADD
or MULTIPLY time with a number or a word. However, we can increase or
decrease time by using its own units.

Similarly, there are tons of different data types in the world with its
own properties, own meaning, own interpretations, and own use case. So,
computers have to be able to use different types of data for storage and
computation.

There are many computer languages, operating systems, systems, etc. in
the world. However, the one that is the most used as data storage system
are databases. Of course the computer hardware stores the data, but how
a data is interpreted decides WHAT data type it is used as.

Hence, it is safe to turn towards database systems to understand the
different data types that one can have, the properties of those data
types, and how they should be interpreted. Now the question is, WHICH
database system should you look at for a comprehensive list of data
types? The answer that I would personally give is to look at Open Source
Database Systems. The advantage of Open Source systems is that the
system is updated by people all over the globe having all types of use
cases. Instead of companies who want to cover only the important use
cases.

At the time of writing this article, the most famous Open Source
Database System is PostgreSQL. You can find a list of different data
types for its version 9.5 easily.

Note:- It is best to look at the most recent version of the database for
an updated list of data types. Also, it is best to get indulged in ALL
the data types as each one might only have a limited use case and some
data types are only conceptual.

In general, the following data types that are what you will see most
commonly being used:

-   Numbers
    -   Integers (Whole Numbers)
    -   Floating Point (Decimal Numbers)
-   Money
-   Characters
    -   Single Character
    -   Multiple Characters
-   Binary
-   Date/Time
-   Boolean (True or False)
-   Enumeration (Static and ordered set of values. Eg:- Days of Week)
-   Geometric Shapes
    -   Points (x, y)
    -   Lines (x1, y1, x2, y2)
    -   Circles
-   Network Addresses
    -   ipv4
    -   ipv6
-   JSON (JavaScript Object Notation)

Of course there are other data types as well with different properties,
and different storage and interpretation formats. But these should be
sufficient to give you an idea of the various types of data available in
Computer Science.

# Primitive V/s Non-Primitive Data Types

The data type that has its set of properties which cannot be further
broken down is known as a Primitive Data type such as integer,
character, time, etc.

A combination of primitive data types with its own set of rules and
properties is known as non-primitive data type.

For example, Name of a person(Words) is a primitive data type, age or a
person(Number) is a primitive data type, and gender of a
person(Character) is a primitive data type.

But, if you combine all three into a new data type, you can call it a
Person data type having the 3 data types as its properties. A Person can
have a name, gender, and age.