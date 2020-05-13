#include<iostream>
template<typename DataType>

/*
This program lists basic array operations limited to setters and getters.

1. Get the no. of elements in the array.
2. Check if the array is empty.
3. Append to the array.
4. Get data at a particular index
5. Put data at a particular index
6. Remove data at a particular index
7. Get index of a particular value
8. Reset the array
9. Add an element at a particular index
*/

class myArray{
    // An array pointer of the specified data type
    DataType* array;
    // Getting the size that a user specified for the array.
    // This is used when we want to dynamically increase the size of the array.
    int incrementSize;
    // The total no. of elements that are inserted in the array
    int totalElements;
    // The actual size of the array
    int size;

    public:

    // Constructor to initialize the variables.
    myArray(int size)
    {
        this->incrementSize=size;
        this->array=new DataType[this->incrementSize];
        this->totalElements=0;
        this->size=size;
    }

    // 1. Get the size of the array
    int getSize(){
        return this->totalElements;
    }
    // 2. Check if an aray is empty
    bool isEmpty(){
        return this->totalElements==0;
    }

    // 3. Add a new element to the array
    // If the array is too small, expand it.
    void append(DataType value)
    {
        if(this->totalElements==this->size)
        {
            this->size=this->size+(this->incrementSize/2);
            DataType* newArray = new DataType[this->size];
            for(int i=0;i<this->size;i++)
                newArray[i]=this->array[i];
            delete[] this->array;
            this->array=newArray;
        }
        this->array[totalElements]=value;
        totalElements++;
    }

    // 4. Get the data at a particular index
    DataType getDataAtIndex(int index)
    {
        if(index<0||index>=totalElements)
            return -1;
        return this->array[index];
    }

    // 5. Set data at a particular index
    bool setDataAtIndex(int index,DataType value)
    {
        if(index<0||index>=totalElements)
            return false;
        this->array[index]=value;
        return true;
    }

    // 6. Remove data from a particular index
    bool removeDataAtIndex(int index)
    {
        if(index<0||index>=totalElements)
            return false;
        
        for(int i=index;i<this->totalElements;i++)
            this->array[i]=this->array[i+1];

        totalElements--;
        return true;
    }

    // 7. Get index of a certain value
    int getIndexOf(DataType value)
    {
        for(int i=0;i<this->totalElements;i++)
            if(this->array[i]==value)
                return i;
        
        return -1;
    }

    // 8. Clear everything that was there previously in an array.
    void clear()
    {
        DataType* newArray = new DataType[this->incrementSize];
        this->totalElements=0;
        delete[] this->array;
        this->array=newArray;
    }

    // 9. Put a new Data value at the specified index
    void putDataAtIndex(int index,DataType value)
    {
        if(index<0||index>=totalElements)
            return;
        this->append(value);
        for(int i=this->totalElements-1;i>index;i--)
            this->array[i]=this->array[i-1];
        this->array[index]=value;
    }
};

// Main function to test all the functions. This can be modified to suit your needs.

int main()
{
    myArray<int> arr(5);
    arr.append(5);
    arr.append(3);
    arr.append(22);
    arr.append(12);
    arr.append(42);
    arr.append(33);

    arr.setDataAtIndex(4,9);

    // Print the details of the array before removing operations.
    std::cout<<"Array Size = "<<arr.getSize()<<std::endl<<"Array Elements: "<<std::endl;
    for(int i=0;i<arr.getSize();i++)
        std::cout<<arr.getDataAtIndex(i)<<", ";
    std::cout<<std::endl;

    arr.removeDataAtIndex(arr.getSize()-1);
    arr.putDataAtIndex(2,999);

    // Print the details of the array after removing operations.
    std::cout<<"Array Size = "<<arr.getSize()<<std::endl<<"Array Elements: "<<std::endl;
    for(int i=0;i<arr.getSize();i++)
        std::cout<<arr.getDataAtIndex(i)<<", ";
    std::cout<<std::endl;

    return 0;
}