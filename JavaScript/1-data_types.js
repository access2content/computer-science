/*    Numbers   */
//  Numbers in JS are 64-bit floating values by default
var num = 5/2;
console.log("Number = ", num);

/*    Strings   */
//  Single quotes and double quotes are the same
var str1 = "My name is Vivek";
var str2 = 'My name is Vivek';
console.log(str1);
console.log(str2);

//  Using Template String to include variables
var age = 69;
console.log(`I am ${age} years old`);

//  Using Template String for multi-line strings.
var multiString = `JavaScript allows
multi line strings.
And they work perfectly fine`;
console.log(multiString);

/*    Null/Undefined   */
//  Default value of a variable is undefined
var undef;
console.log("Value of Undefined:",undef);

//  Manually assigning null to a variable
var nu = null;
console.log("Value of Null:",nu);

//  Null and Undefined are the same
console.log("Null = Undefined:",nu == undef);

/*    Typeof   */
var obj = { name: "Vivek", age: 69}
var arr = [1,2,3];
console.log("Typeof Undefined:", typeof undef);
console.log("Typeof Number:", typeof num);
console.log("Typeof String:", typeof str1);
console.log("Typeof Object:", typeof obj);
console.log("Typeof Number:", typeof nu);
console.log("Typeof Array:", typeof arr);
console.log("IsArray:", Array.isArray(arr));

/*    Object   */
let cap={
	name: "Steve",
	age: 69,
	isAvenger: true
}

for(let key in cap){
	console.log(key, "-", cap[key], "-", cap.key);
}
