- [Execution Context (EC)](#execution-context-ec)
	- [Phases of an EC](#phases-of-an-ec)
		- [Creation](#creation)
		- [Execution](#execution)
- [var vs let](#var-vs-let)
- [Call Stack](#call-stack)
- [Key Points](#key-points)
- [References](#references)


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

[Execution Context](Execution%20Context.md)
# var vs let
- `var` keyword creates a function scoped variable
- `let` keyword creates block/script scoped variable

If a variable is initiated using var keyword, it can be re-declared in the same scope whereas a variable with let cannot.

There is a difference between them when the memory is created for them. In GEC, `var` goes to the global scope whereas `let` goes to the script scope.
# Call Stack
The first thing that gets pushed into the call stack is the Global Execution Context (GEC).

# Key Points
- The code that isn't inside any function is in the global scope.
- The features are given by the environment, only the logic can be written in JS.

# References
- [Will Sentance](https://www.youtube.com/watch?v=exrc_rLj5iw)