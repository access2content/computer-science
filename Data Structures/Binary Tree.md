- [Observations](#observations)
- [Implementation](#implementation)
	- [Array Implementation](#array-implementation)
	- [Linked List Implementation](#linked-list-implementation)

A tree where each Node can have at most 2 children is known as a Binary Tree.

# Observations
- `The Maximum no. of Nodes at level i is 2^i.`
- `The Max. no. of nodes at height h is 2^h - 1`

# Types of Binary Trees
Depending on how nodes are arranged in a binary tree, it can be full, complete and perfect:

- **Full binary tree**: Each node has exactly 0 or 2 children (but never 1).
- **Complete binary tree**: When all levels except the last one are full with nodes.
- **Perfect binary tree**: when all the levels (including the last one) are full of nodes.

![](structures/Types%20of%20Binary%20Trees.jpg)

# Applications
- Expression Trees in Compilers
- Huffman coding trees in data compression algorithms
- Priority Queue is another application of binary tree that is used for searching maximum or minimum in O(1) time complexity
- Represent hierarchical data
- Editing software like Microsoft Excel and spreadsheets
- Indexing segmented at the database is useful in storing cache in the system
- Syntax trees are used for most famous compilers for programming like GCC, and AOCL to perform arithmetic operations
- Priority Queues
- Find elements in less time (binary search tree)
- Enable fast memory allocation in computers. 
- Perform encoding and decoding operations.
- Organize and retrieve information from large datasets, such as in inverted index and k-d trees.
- Represent the decision-making process of computer-controlled characters in games, such as in decision trees.
 - Implement searching algorithms, such as in binary search trees which can be used to quickly find an element in a sorted list.
- Implement sorting algorithms, such as in heap sort which uses a binary heap to sort elements efficiently.

# Implementation
We can implement Trees via Arrays as well as Linked Lists.

## Array Implementation
The left child, will be at `2*i`th index.
The right child will be at `2*i - 1`th index.
The parent of each child we be at `n/2`th index.

## Linked List Implementation
```java
import java.util.Scanner;

public class Tree{
	static Scanner sc = null;
	public static void main(String[] args){
		sc = new Scanner(System.in);

		createTree();
	}

	static Node createTree(){
		Node root = null;

		System.out.println("Enter Data: ");
		int data = sc.nextInt();
		if(data == -1) return null;

		root = new Node(data);
		System.out.println("Enter left for "+ data);
		root.left = createTree();

		System.out.println("Enter right for "+ data);
		root.right = createTree();
		
		return root;
	}
}

class Node{
	int data;
	Node left, right;

	public Node(int data){
		this.data = data;
	}
}
```

# Traversal
## In-order Traversal
Left-Node-Right
## Pre-order Traversal
Node-Left-Right
## Post-order Traversal
Left-Right-Node
# Notes
- Recursion is used a lot in Binary Trees