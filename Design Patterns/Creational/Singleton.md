The Singleton Design Pattern aims to keep a check on initialization of objects of a particular class by ensuring that only one instance of the object exists throughout the Java Virtual Machine.

A Singleton class also provides one unique global access point to the object so that each subsequent call to the access point returns only that particular object.

There will only be a single instance created of this object. All the data related to this instance is stored in this object itself. 

# Assumption
This assumes that you will only ever need a single instance of the class. It is impossible to create a second instance.

# Notes
- If a class has a public constructor, it cannot become a Singleton
# Cons
Since only a single instance can be created for this object, there are some problems that it encounters:
 - Testing becomes a challenge as this instance is required for all tests as this is a global instance and shared by all other modules. So, mocking that becomes almost impossible.
 - It creates tight coupling between various parts of the application and hence it becomes hard to change or refactor as entire application is dependent on it.
 - There will be race conditions as multiple instances will try to write data to this instance at the same time.
 - Since the scope is global and anyone can change it, it leads to unexpected behavior of your code.
 - Data Inconsistency
 - Hidden Dependencies
 - Breaks SOLID principles - Does not have single responsibility (Managing its lifecycle)
 - High-level modules should not depend on low level details. Instead, both of them should depend on abstractions.

# Pros
Some reasons why there are problems with Singleton is because:
- Size of projects
- Encouragement of Concurrency
- Automated Testing
- Perception towards Singleton as data source

# Where it is used?
- When only one instance is required and 2nd instance might be dangerous
- When there is a shared resource such as Logging, Driver Objects, Caching, and Thread Pool, Database Connection Management, etc.
- If a class is stateless
- When objects are expensive to create
- When objects are immutable
- Configuration Management
- In other design patterns like Abstract Factory, Builder, Prototype, Facade, etc.
# Implementation
To implement a singleton pattern, we have different approaches, but all of them have the following common concepts.
- Private constructor to restrict instantiation of the class from other classes.
- Private static variable of the same class that is the only instance of the class.
- Public static method that returns the instance of the class, this is the global access point for the outer world to get the instance of the singleton class.

Let's look at the different approaches to singleton pattern implementation and design concerns with the implementation.
## 1. Eager Initialization
The instance of the singleton class is created at the time of class loading. 

The drawback to eager initialization is that the method is created even though the client application might not be using it.

```java
public class EagerInitializedSingleton {

    private static final EagerInitializedSingleton instance = new EagerInitializedSingleton();

    // private constructor to avoid client applications using the constructor
    private EagerInitializedSingleton(){}

    public static EagerInitializedSingleton getInstance() {
        return instance;
    }
}
```

If your singleton class is not using a lot of resources, this is the approach to use.
Also, this method doesn’t provide any options for exception handling.

## 2. Static Block Initialization
It is similar to eager initialization, except that instance of the class is created in the static block that provides the option for exception handling.

```java
public class StaticBlockSingleton {

    private static StaticBlockSingleton instance;

    private StaticBlockSingleton(){}

    // static block initialization for exception handling
    static {
        try {
            instance = new StaticBlockSingleton();
        } catch (Exception e) {
            throw new RuntimeException("Exception occurred in creating singleton instance");
        }
    }

    public static StaticBlockSingleton getInstance() {
        return instance;
    }
}
```

## 3. Lazy Initialization
The singleton pattern creates the instance in the global access method as and when required instead of creating when the class is initialized.

```java
public class LazyInitializedSingleton {

    private static LazyInitializedSingleton instance;

    private LazyInitializedSingleton(){}

    public static LazyInitializedSingleton getInstance() {
        if (instance == null) {
            instance = new LazyInitializedSingleton();
        }
        return instance;
    }
}
```

**Note**: This implementation works fine in the case of the single-threaded environment, but when it comes to multi-threaded systems, it can cause issues if multiple threads are inside the if condition at the same time.

## 4. Thread Safe Initialization
A simple way to create a thread-safe singleton class is to make the global access method synchronized so that only one thread can execute this method at a time.

```java
public class ThreadSafeSingleton {

    private static ThreadSafeSingleton instance;

    private ThreadSafeSingleton(){}

    public static synchronized ThreadSafeSingleton getInstance() {
        if (instance == null) {
            instance = new ThreadSafeSingleton();
        }
        return instance;
    }

}
```

This implementation works fine and provides thread-safety, but it reduces the performance because of the cost associated with the synchronized method. To avoid this extra overhead every time, *double-checked locking* principle is used. In this approach, the synchronized block is used inside the if condition with an additional check to ensure that only one instance of a singleton class is created.

```java
public static ThreadSafeSingleton getInstanceUsingDoubleLocking() {
    if (instance == null) {
        synchronized (ThreadSafeSingleton.class) {
            if (instance == null) {
                instance = new ThreadSafeSingleton();
            }
        }
    }
    return instance;
}
```

## 5. Bill Pugh Singleton Implementation
The previous approaches used to fail in certain scenarios where too many threads tried to get the instance of the singleton class simultaneously. So Bill Pugh came up with a different approach to create the singleton class using an inner static helper class.

```java
public class BillPughSingleton {

    private BillPughSingleton(){}

    private static class SingletonHelper {
        private static final BillPughSingleton INSTANCE = new BillPughSingleton();
    }

    public static BillPughSingleton getInstance() {
        return SingletonHelper.INSTANCE;
    }
}
```

Notice the private inner static class that contains the instance of the singleton class. When the singleton class is loaded, SingletonHelper class is not loaded into memory.

Only when someone calls the getInstance() method, this class gets loaded and creates the singleton class instance. This is the most widely used approach for the singleton class as it doesn’t require synchronization.
# Additional Reading
- [DZone Singleton](https://dzone.com/articles/singleton-design-pattern-%E2%80%93)
- [Digital Ocean - Singleton Best Practices](https://www.digitalocean.com/community/tutorials/java-singleton-design-pattern-best-practices-examples)