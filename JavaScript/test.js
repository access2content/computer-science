console.log(globalThis == global);

var a = 10; 

function bar(){
    function foo(){
        console.log(a);
    };

    var a = 20; 
    foo();
};

bar();