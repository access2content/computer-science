All code is executed inside the Execution Context(EC). Firstly, the EC is created, only then the code is executed.

The code that isn't inside any function is in the global area. It executes in the Global Execution Context.

```javascript
let a=10;
function fn(){
	console.log("I am fn");
	function inner(){
		console.log("I am inner");
	}
	inner();
}
fn();
```

Challenge:
```javascript
function real(){
	console.log("I am the real function.");
}
function real(){
	console.log("No, I am definitely the real one.");
}
real();
function real(){
	console.log("Oh please!");
}
```

# Execution Context (EC)
JavaScript Execution Context refers to the environment that allows JavaScript code to be executed. Everything in JavaScript happens inside the Execution Context.

Anytime a function is called, its execution Context is created. In recursive functions, a new execution context is created each time the function calls itself.

This EC is then pushed on to the Call Stack.

There are 3 types of EC:
- Global Execution Context
- Local/Function Execution Context
- Eval Function Execution Context

There are 2 components of the EC:
- Variable Environment - All variables and functions are stored as key value pairs
- Thread of Execution - Code is executed one line at a time
## Phases of an EC
### Creation
In this phase, the EC does the following:
- Create the global object i.e., `window` in the web browser or `global` in Node.js.
- Create the `this` object and bind it to the global object.
- Setup a memory heap for storing variables and function references.
- Store the function declarations in the memory heap and variables within the global execution context with the initial values as `undefined`.

Hoisting is a fancy term for Memory Allocation. 
- Any variable that is declared initially is given the value `undefined`.
- Functions get memory allocated to them

```javascript
console.log("Val of a before:", a);
var a=10;
console.log("Val of a after:", a);
```

In this code, one can see that a has undefined assigned to it before the real value is assigned.
### Execution
The code is executed line by line.

# var vs let
- `var` keyword creates a function scoped variable
- `let` keyword creates block/script scoped variable

If a variable is initiated using var keyword, it can be re-declared in the same scope whereas a variable with let cannot.

# Call Stack
The first thing that gets pushed into the call stack is the Global Execution Context (GEC).

# Key Points
- The code that isn't inside any function is in the global scope.
- The features are given by the environment, only the logic can be written in JS.