class Singleton{
    private static instance: Singleton;

    private constructor(){};

    public static getInstance(): Singleton{

        if(!Singleton.instance){
            Singleton.instance = new Singleton();
        }
        return Singleton.instance;
    }

}

function client(){
    const one = Singleton.getInstance();
    const two = Singleton.getInstance();

    if(one === two) console.log("Both are same Bhidu");
    else    console.error("Both are different!!!");
}

client();