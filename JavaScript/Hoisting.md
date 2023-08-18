Hoisting is a fancy term for Memory Allocation. 
- Any variable that is declared initially is given the value `undefined`.
- Functions get memory allocated to them

```javascript
console.log("Val of a before:", a);
var a=10;
console.log("Val of a after:", a);
```

In this code, one can see that a has undefined assigned to it before the real value is assigned.