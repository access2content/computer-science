const input = require("readline-sync");

class Node{
    constructor(data){
        this.data = data;
        this.left = null;
        this.right = null;
    }
}

let root = createTree();
console.log("Pre-order");
preorder(root);
console.log("In-order");
inorder(root);
console.log("Post-order");
postorder(root);

//  Returns the root Node of the new tree
function createTree(){
    let nodeData = input.question("Enter data: ");
    if(nodeData == -1) return null;

    let root = new Node(nodeData);

    console.log(`Left sub-tree for ${nodeData}: `);
    root.left = createTree();

    console.log(`Right sub-tree for ${nodeData}: `);
    root.right = createTree();

    return root;
}

function preorder(root){
    if(root == null) return;

    console.log(root.data+" ");
    preorder(root.left);
    preorder(root.right);
}

function postorder(root){
    if(root == null) return;

    postorder(root.left);
    postorder(root.right);    
    console.log(root.data+" ");
}

function inorder(root){
    if(root == null) return;

    inorder(root.left);
    console.log(root.data+" ");
    inorder(root.right);
}