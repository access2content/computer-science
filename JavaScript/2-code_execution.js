/*	Trick Question	*/
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

/*  Hoisting   */
//	Creating variables using var "function scoped"
console.log("Val of a before:", a);
var a=10;
console.log("Val of a after:", a);

function fn(){
	console.log("a inside fn:", a);
	var a = 20;
	a++;
	console.log("After increment:", a);

	if(a){
		var a = 30;
		a++;
		console.log("Inside if:", a);
	}
	console.log("Outside if:", a);
}
fn();
console.log("After everything:", a);

//	Creating variables using let "block scoped"
let b=10;
console.log("Val of b after:", b);

function fn(){
	//	Error because b is still not created
	// console.log("b inside fn:", b);
	let b = 20;
	b++;
	console.log("After increment:", b);

	if(b){
		let b = 30;
		b++;
		console.log("Inside if:", b);
	}
	console.log("Outside if:", b);
}
fn();
console.log("After everything:", b);

//	Shadowing
let fruits = "apple";
console.log("Main:", fruits);
{
	//	Error because fruit not created inside block
	// console.log("Inside Block:", fruits);
	let fruits;
	console.log("After redeclare:", fruits);
	fruits = "orange";
	{
		console.log("Inside empty bracket:", fruits);
	}
	console.log("After re-assign:", fruits);
}

console.log("Final:", fruits);
