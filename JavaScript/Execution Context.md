- [Introduction](#introduction)
- [Types of Execution Contexts](#types-of-execution-contexts)
	- [Global Execution Context(GEC)](#global-execution-contextgec)
	- [Functional Execution Context](#functional-execution-context)
	- [Eval Execution Context](#eval-execution-context)
- [Creating an Execution Context](#creating-an-execution-context)
	- [Variable Environment](#variable-environment)
	- [Lexical Environment](#lexical-environment)
		- [Environment Record](#environment-record)
			- [Declarative Environment Record](#declarative-environment-record)
			- [Object Environment Record](#object-environment-record)
			- [Global Environment Record](#global-environment-record)
		- [Reference to outer Lexical Environment](#reference-to-outer-lexical-environment)
		- [`this` binding](#this-binding)
- [Executing an Execution Context](#executing-an-execution-context)
- [Thread of Execution?](#thread-of-execution)
- [Key Points](#key-points)
- [References](#references)

# Introduction
Execution Context(EC) refers to the environment that allows JavaScript code to be executed. it is an abstract concept that represents the environment in which JavaScript runs. Everything in JavaScript happens inside the Execution Context. 

Understanding EC helps you understand other important concepts in JavaScript, such as:
- Hoisting
- Scope
- Call Stack
- Scope Chain
- Closure
- Event Loop

# Types of Execution Contexts
There are 3 types of Execution Contexts:
- [Global Execution Context(GEC)](#global-execution-contextgec)
- [Functional Execution Context](#functional-execution-context)
- [Eval Execution Context](#eval-execution-context)

## Global Execution Context(GEC)
This is the default or base execution context. The code that is **not inside any function** is in the global execution context.

Global Execution Context (GEC) is located at the bottom of the execution stack(call stack), and it is included by default. 

A Global Execution Context is created as follows:

- Create the global object i.e., `window` in the web browser or `global` in Node.js.
- Create the `this` object and bind it to the `globalThis` object.
- Create a memory heap in order to store variables and function references.
- Stores all the functions declarations in the memory heap area and the variables in the GEC with initial values as 'undefined'.

==Note: There can only be one Global Execution Context in a program.==
## Functional Execution Context
Each function has its own execution context, and it’s created when the function is invoked or called. In recursive functions, a new execution context is created each time the function calls itself.

## Eval Execution Context
Code executed inside an eval function also gets its own execution context, but as eval isn’t usually used by JavaScript developers.

# Creating an Execution Context
During the creation phase, two-state components are created:
- Lexical Environment
- Variable Environment

```javascript
ExecutionContext = {
  ThisBinding: <this value>,
  LexicalEnvironment: { ... }
  VariableEnvironment: { ... },
}
```

==Conceptual representation of an Execution Context==

The Lexical Environment stores identifier bindings to the values of variables (let and const) and functions, whereas the Variable Environment simply stores identifier bindings to the values of the variable (var).

## Variable Environment
It is a Lexical Environment that defines the relationship between identifiers and the values of variables, but not functions.
## Lexical Environment
A lexical environment is a structure that holds identifier-variable mapping. Here, identifier refers to the name of variables/functions, and the variable is the reference to actual object in memory.

It has 3 components:
- Environment Record
- `this` binding
- reference to the outer lexical environment - 
### Environment Record
The environment record is the place where the variable and function declarations are stored.

There are three main types of environment records:
- [Declarative Environment Record](#declarative-environment-record)
- [Object Environment Record](#object-environment-record)
- [Global Environment Record](#global-environment-record)

The other types of environment record are:
- Function Environment Record
- Module Environment Record
#### Declarative Environment Record
Variables, classes, modules, and/or function declarations are all stored here.
The lexical environment for function code contains a declarative environment record.

#### Object Environment Record
Global variables and functions are added to the global execution context's object environment record. The `arguments` object and `this` object make up the object environment record in the local/function execution context.

#### Global Environment Record
Although a global Environment Record is theoretically a single record, it is stated as a composite encapsulating both an object and a declarative Environment Record. It has no exterior environment because its `OuterEnv` is null.

It includes an associated global object whose properties offer some of the global environment's identifier bindings, and it may be prepopulated with identifier bindings. Additional attributes may be added to the global object as JavaScript code is executed, and the initial properties may be updated.

### Reference to outer Lexical Environment
The JavaScript engine can look for variables in the outer environment if they are not found in the current lexical environment using this reference.

During execution, Lexical Environment is used to resolve the bindings of values. If a value is not found in the current Environment Record, JS searches for it in the outer lexical environment till it can't find it in the GEC. The outer reference for GEC is null, so, if it not found till GEC, the identifier is marked as not found. 

This process is called **identifier resolution** and happens on running Execution Context.

### `this` binding
In this component, the value of `this` is determined or set.

In the global execution context, the value of `this` refers to the `global` or `window` object.

In the function execution context, the value of `this` depends on how the function is called. If it is called by an object reference, then the value of `this` is set to that object, otherwise, the value of `this` is set to the `globalThis` object or undefined (in strict mode).

# Executing an Execution Context
Variable binding initialization, variable assignments, mutability and immutability checking, variable binding deletions, function call execution, and so on are all occurring during this phase.
# Thread of Execution?

# Key Points
- Anytime a function is called, its EC is created. 
- Everything in JavaScript happens inside the Execution Context. 

# References
- [Atatus - Complete Guide for JS Execution Context](https://www.atatus.com/blog/javascript-execution-context/)
- [JS Behind the Scenes](https://www.freecodecamp.org/news/execution-context-how-javascript-works-behind-the-scenes/)
- [Understanding Execution Context](https://blog.bitsrc.io/understanding-execution-context-and-execution-stack-in-javascript-1c9ea8642dd0)