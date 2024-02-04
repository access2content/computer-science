const input = require("readline-sync");

class TreeNode{
  data: number;
  left: TreeNode | null;
  right: TreeNode | null;

    constructor(data: number){
        this.data = data;
        this.left = null;
        this.right = null;
    };

    static createTree(): TreeNode | null{
        let nodeData: number = input.question("Enter data: ");
        if(nodeData == -1) return null;

        let root: TreeNode|null = new TreeNode(nodeData);

        console.log(`Left sub-tree for ${nodeData}: `);
        root.left = this.createTree();

        console.log(`Right sub-tree for ${nodeData}: `);
        root.right = this.createTree();

        return root;
    }

    preorder(){
        if(this == null) return;

        console.log(this.data+" ");
        this.left?.preorder();
        this.right?.preorder();
    }

    postorder(){
        if(this == null) return;

        this.left?.postorder();
        this.right?.postorder();    
        console.log(this.data+" ");
    }

    inorder(){
        if(this == null) return;

        this.left?.inorder();
        console.log(this.data+" ");
        this.right?.inorder();
    }
}

function main(){
  let root: TreeNode | null = TreeNode.createTree();
  if (!root) return;

  console.log("Pre-order");
  root.preorder();
  console.log("In-order");
  root.inorder();
  console.log("Post-order");
  root.postorder();
}

main();
