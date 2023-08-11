# Data Types
JS supports both Primitive and non-primitive data types. In 2015, ES6 was launched that introduced new types.
## Primitive
Primitive data types are types that cannot be further divided. JS supports the following data types:
- string
- number
- null
- boolean
- undefined
- BigInt (ES6)
- Symbol (ES6)
## Non-Primitive
- Objects
- Functions
- Arrays
- Map (ES6)
- Set (ES6)
- WeakMap (ES6)
- WeakSet (ES6)
# Variables
Variables in JS are dynamically typed.
## Numbers
Numbers in JS are 64-bit floating values by default.
## Strings
There are no characters in JS, only strings.
There is no difference between single quotes (') and double quotes (") in JS to create strings.

Using quotes, one cannot create multi line strings. So, template strings was introduced which used the back-tick operator (\`).
## Null/Undefined
The default value of a variable is always `undefined`.

`null` and `undefined` both represent an empty value. The difference is that undefined is assigned by JavaScript whereas if a developer wants to manually assign an empty value, they would use null.

# Typeof
Since JS is a dynamically typed language, a variable can change its data type at runtime. Hence, the operator `typeof` allows us to infer the data type of the variable being used. It returns a string that specifies the data type of the given variable.

JS stores the type of variable as follows:
- 000 - Reference to Object
- 1 - 31 bit Integer Data
- 010 - double. Reference to Double Floating Point Number
- 100 - string. Reference to a String
- 110 - boolean. Boolean Data

Further Reading: [The history of “typeof null”](https://2ality.com/2013/10/typeof-null.html)

## typeof null
When using the operator for various data types, the `typeof` returns "object" for null. This is because the variable type is stored a 000 for `null`.

## typeof array
The typeof of array is object. Since we have an array and not an object, we need a way to check if the data is array. So, we will check it using the `Array.isArray()` function.
```javascript
console.log(Array.isArray([1,2,3]));
```

# Object
An object is a collection of key and value pairs. The key can either be a number or a string. The value can be any valid JavaScript. For example:
```javascript
let cap = {
	name: "Steve",
	"Last Name": "Rogers",
	isAvenger: true,
	address:{
		city: Chandigarh
	},
	sayHi: function(){console.log("Hello world")},
	movies: ["Avenger", "Civil War"]
}
```

There are 2 ways to reference a key of an object:
- Dot Operator `cap.name`
- Bracket Operator `cap['Last Name']`

The dot operator is a literal operator whereas the bracket operator uses the value passed inside it as the key value.

```javascript
let cap={
	name: "Steve",
	age: 69,
	isAvenger: true
}

for(let key in cap){
	console.log(key, "-", cap[key], "-", cap.key);
}
```

Practice questions:
- [Profile Lookup](https://www.freecodecamp.org/learn/javascript-algorithms-and-data-structures/basic-javascript/profile-lookup)
- [Record Collection](https://www.freecodecamp.org/learn/javascript-algorithms-and-data-structures/basic-javascript/record-collection)
# Key Points
- Default value of a variable is `undefined`.
- typeof null is object
- typeof array is object