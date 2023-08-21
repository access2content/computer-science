These is my learning from the masterclass at Scaler for Backtracking. This was taught by Rama sir.

# Pre-requisiste
- Any programming language
- Recursion
# What is Backtracking?
Generating all possible solutions. When generating all solutions, choose a path till we cannot go any further. Then, go back and take another path.

# How do you know to use Backtracking?
Wherever you see ALL solutions, it has a possibility of backtracking.

# Steps
- Parameters - Decide what needs to be passed to function. Result, start, end is a must
- Choices - At any state, what are the choices?
- Return Type - What needs to be returned

# Questions
## Given N, print all N digit numbers formed only using 1 or 2 in increasing order.
Eg:
N = 2,
11, 12, 21, 22

Take an array of size N. Start with the 0th index. There are 2 options - 1 or 2.

- Parameters - array[], i, N
- Choices - 1 or 2
### Code
```c++
void printAll(int ar[], int i, int N){
	if(i ==N){
		// Print the array
		return;
	}
	
	// At ith index, path 1
	ar[i] = 1;
	printAll(ar, i+1, N);

	// At ith index, path 2
	ar[i] = 2;
	printAll(ar, i+1, N);

	return;
}
```

## Given `N*N` matrix, print all valid placements on N queens so that no queen can kill each other.

- Parameters - `array[n][n]`, row, maxQueens
- Choices - all cells in the current row
- Return Type - 

### Code

```c++

bool valid(int mat[][], i, j){
	int x = i, y = j;
	// Current Column
	while(x>0){
		if(mat[x][y] == 1)
			return false;
		x--;
	}

	// Left Diagonal
	x = i, y = j;
	while(x>0 && y>0){
		if(mat[x][y] == 1)
			return false;
		x--, y--;
	}

	// Right Diagonal
}

void nQueens(int mat[][], int i, int N){

	if(i==N){
		print(mat);
		return;
	}

	for(int j = 0; j<N; j++){
		if(valid(mat,i,j,N)){
			mat[i][j] = 1;
			nQueens(mat, i+1, N);
			mat[i][j] = 0;
		}
	}

	return;
}
```