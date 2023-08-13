// let cap={
// 	firstName: "Steve",
// 	sayHi: function(){
// 		console.log("Hi from", this.firstName);
//         const inner = function(){
//             console.log("Inner", this.firstName);
//         }
//         inner();
// 	}
// }

// cap.sayHi();
// // let sayHiAdd = cap.sayHi;
// // var firstName = "Vivek";
// // sayHiAdd();
// // console.log(this);
// this.firstName = 'Test';

let cap2={
	firstName: "Steve",
	sayHi: ()=>{
		console.log("Hi from", this.firstName);
        const inner = ()=>{
            console.log("Inner", this.firstName);
        }
        inner();
	}
}

cap2.sayHi();