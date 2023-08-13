- [Topics](#topics)
- [how this behaves in JS](#how-this-behaves-in-js)
- [chaining in JS](#chaining-in-js)
- [bind, call, and apply](#bind-call-and-apply)
- [prototypal inheritance in JS](#prototypal-inheritance-in-js)
- [polyfills of bind, call, and apply](#polyfills-of-bind-call-and-apply)

# Topics
- [ ] this
- [ ] bind
- [ ] call
- [ ] apply

Q. Diff b/w native and host object
Native Object: The object that you get from environment such as window, global. They are not part of JS specification.

Host: The objects given by language such as JSON, Date, Object, etc.
# how this behaves in JS
This behaves differently in:
- strict mode
- function
- arrow function

```javascript
let cap={
	firstName: "Steve",
	sayHi: function(){
		console.log("Hi from", this.firstName);
	}
}

cap.sayHi();
let sayHiAdd = cap.sayHi;
sayHiAdd();
```

For Global Execution Context, `this` will be `window`.
When a function is invoked on an Object, `this` refers to that Object.
For a function invoked without Object, `this` will be `global`.
For a function invoked inside arrow function, `this` will be copied from the Execution context where it is declared.

Every time an Execution Context is created, `this` is calculated.
# Strict mode
- for function call, `this` will be undefined.
# chaining in JS
return `this` for all function call to enable chaining.

# bind, call, and apply

# prototypal inheritance in JS

# polyfills of bind, call, and apply