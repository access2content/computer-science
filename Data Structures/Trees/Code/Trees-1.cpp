#include<iostream>
using namespace std;

class Node{
    public:

    int data;
    Node* left;
    Node* right;

    Node(int data){
        data = data;
        left = NULL;
        right = NULL;
    }
};

Node* createTree(){
    int nodeData;
    cout<<"Enter Data: ";
    cin>>nodeData;

    if(nodeData == -1) return NULL;

    Node* root = new Node(nodeData);

    cout<<"Left Sub-tree for "<<nodeData<<endl;
    root->left = createTree();

    cout<<"Right Sub-tree for "<<nodeData<<endl;
    root->right = createTree();
    
    return root;
}

void inorder(){

}

void preorder(){

}

void postorder(){

}

int main(){
    Node* root = createTree();
    cout<<root;

    return 0;
}