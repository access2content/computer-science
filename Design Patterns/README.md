- [Introduction](#introduction)
- [Types of design Patterns](#types-of-design-patterns)
	- [Creational Design Patterns](#creational-design-patterns)
	- [Structural Design Patterns](#structural-design-patterns)
	- [Behavioral Patterns](#behavioral-patterns)
	- [Concurrency Patterns](#concurrency-patterns)
	- [Data Access Patterns](#data-access-patterns)
	- [Real-Time Patterns](#real-time-patterns)

# Introduction
In software engineering, a design pattern is a general reusable solution to a commonly occurring problem in software design. A design pattern is not a finished design that can be transformed directly into code. It is a description or template for how to solve a problem that can be used in many different situations. Object-oriented design patterns typically show relationships and interactions between classes or objects, without specifying the final application classes or objects that are involved.

These are very important.
# Why learn?
- Acts like a Shared Vocabulary
- Saves time
# Types of design Patterns
Design patterns are divided into 3 main categories:
## [Creational Design Patterns](Creational/README.md)
- [Singleton](Creational/Singleton.md): Ensure a class has only one instance, and provide a global point of access to it.
- [Factory](Creational/Factory.md): Define an interface for creating an object, but let subclasses decide which class to instantiate. Factory Method lets a class defer instantiation to subclasses.
- [Abstract Factory](Creational/Abstract%20Factory.md): Provide an interface for creating families of related or dependent objects without specifying their concrete classes.
- [Builder](Creational/Builder.md): Separate the construction of a complex object from its representation allowing the same construction process to create various representations.
- Lazy initialization: Tactic of delaying the creation of an object, the calculation of a value, or some other expensive process until the first time it is needed.
- Multiton: Ensure a class has only named instances, and provide global point of access to them.
- Object pool: Avoid expensive acquisition and release of resources by recycling objects that are no longer in use. Can be considered a generalisation of connection pool and thread pool patterns.
- Prototype: Specify the kinds of objects to create using a prototypical instance, and create new objects by copying this prototype.
- Resource acquisition is initialization: Ensure that resources are properly released by tying them to the lifespan of suitable objects.

## [Structural Design Patterns](Structural/README.md)
- Adapter or Wrapper: Convert the interface of a class into another interface clients expect. Adapter lets classes work together that could not otherwise because of incompatible interfaces.
- Bridge: Decouple an abstraction from its implementation allowing the two to vary independently.
- Composite: Compose objects into tree structures to represent part-whole hierarchies. Composite lets clients treat individual objects and compositions of objects uniformly.
- Decorator: Attach additional responsibilities to an object dynamically keeping the same interface. Decorators provide a flexible alternative to subclassing for extending functionality.
- Facade: Provide a unified interface to a set of interfaces in a subsystem. Facade defines a higher-level interface that makes the subsystem easier to use.
- Front Controller: Provide a unified interface to a set of interfaces in a subsystem. Front Controller defines a higher-level interface that makes the subsystem easier to use.
- Flyweight: Use sharing to support large numbers of fine-grained objects efficiently.
- Proxy: Provide a surrogate or placeholder for another object to control access to it.

## [Behavioral Patterns](Behavioral/README.md)
- Blackboard: Generalized observer, which allows multiple readers and writers. Communicates information system-wide.
- Chain of responsibility: Avoid coupling the sender of a request to its receiver by giving more than one object a chance to handle the request. Chain the receiving objects and pass the request along the chain until an object handles it.
- Command: Encapsulate a request as an object, thereby letting you parameterize clients with different requests, queue or log requests, and support undoable operations.
- Interpreter: Given a language, define a representation for its grammar along with an interpreter that uses the representation to interpret sentences in the language.
- Iterator: Provide a way to access the elements of an aggregate object sequentially without exposing its underlying representation.
- Mediator: Define an object that encapsulates how a set of objects interact. Mediator promotes loose coupling by keeping objects from referring to each other explicitly, and it lets you vary their interaction independently.
- Memento: Without violating encapsulation, capture and externalize an object's internal state allowing the object to be restored to this state later.
- Null object: Avoid null references by providing a default object.
- Observer or Publish/subscribe: Define a one-to-many dependency between objects where a state change in one object results with all its dependents being notified and updated automatically.
- Servant: Define common functionality for a group of classes
- Specification: Recombinable business logic in a boolean fashion
- State: Allow an object to alter its behavior when its internal state changes. The object will appear to change its class.
- Strategy: Define a family of algorithms, encapsulate each one, and make them interchangeable. Strategy lets the algorithm vary independently from clients that use it.
- Template method: Define the skeleton of an algorithm in an operation, deferring some steps to subclasses. Template Method lets subclasses redefine certain steps of an algorithm without changing the algorithm's structure.
- Visitor: Represent an operation to be performed on the elements of an object structure. Visitor lets you define a new operation without changing the classes of the elements on which it operates.

## Concurrency Patterns
- Active Object: Decouples method execution from method invocation that reside in their own thread of control. The goal is to introduce concurrency, by using asynchronous method invocation and a scheduler for handling requests.
- Balking: Only execute an action on an object when the object is in a particular state.
- Binding Properties: Combining multiple observers to force properties in different objects to be synchronized or coordinated in some way.
- Messaging pattern: The messaging design pattern (MDP) allows the interchange of information (i.e. messages) between components and applications.
- Double-checked locking: Reduce the overhead of acquiring a lock by first testing the locking criterion (the 'lock hint') in an unsafe manner; only if that succeeds does the actual lock proceed. Can be unsafe when implemented in some language/hardware combinations. It can therefore sometimes be considered an anti-pattern.
- Event-based asynchronous: Addresses problems with the Asynchronous pattern that occur in multithreaded programs.
- Guarded suspension: Manages operations that require both a lock to be acquired and a precondition to be satisfied before the operation can be executed.
- Lock: One thread puts a "lock" on a resource, preventing other threads from accessing or modifying it.
- Monitor object: An object whose methods are subject to mutual exclusion, thus preventing multiple objects from erroneously trying to use it at the same time.
- Reactor: A reactor object provides an asynchronous interface to resources that must be handled synchronously.
- Read-write lock: Allows concurrent read access to an object but requires exclusive access for write operations.
- Scheduler: Explicitly control when threads may execute single-threaded code.
- Thread pool: A number of threads are created to perform a number of tasks, which are usually organized in a queue. Typically, there are many more tasks than threads. Can be considered a special case of the object pool pattern.
- Thread-specific storage: Static or "global" memory local to a thread.

## Data Access Patterns
- ORM Patterns: Domain Object Factory, Object/Relational Map, Update Factory
- Resource Management Patterns: Resource Pool, Resource Timer, Retryer, Paging Iterator
- Cache Patterns: Cache Accessor, Demand Cache, Primed Cache, Cache Collector, Cache Replicator
- Concurrency Patterns: Transaction, Optimistic Lock, Pessimistic Lock

## Real-Time Patterns
- Architecture Patterns: Layered Pattern, Channel Architecture Pattern, Component-Based Architecture, Recursive Containment Pattern and Hierarchical Control Pattern, Microkernel Architecture Pattern, Virtual Machine Pattern
- Concurrency Patterns: Message Queuing Pattern, Interrupt Pattern, Guarded Call Pattern, Rendezvous Pattern, Cyclic Executive Pattern, Round Robin Pattern
- Memory Patterns: Static Allocation Pattern, Pool Allocation Pattern, Fixed Sized Buffer Pattern, Smart Pointer Pattern, Garbage Collection Pattern, Garbage Compactor Pattern
- Resource Patterns: Critical Section Pattern, Priority Inheritance Pattern, Priority Ceiling Pattern, Simultaneous Locking Pattern, Ordered Locking Pattern
- Distribution Patterns: Shared Memory Pattern, Remote Method Call Pattern, Observer Pattern, Data Bus Pattern, Proxy Pattern, Broker Pattern
- Safety and Reliability Patterns: Monitor-Actuator Pattern, Sanity Check Pattern, Watchdog Pattern, Safety Executive Pattern, Protected Single Channel Pattern, Homogeneous Redundancy Pattern, Triple Modular Redundancy Pattern, Heterogeneous Redundancy Pattern
# Further Reading
- [Wikibooks](https://en.wikibooks.org/wiki/Introduction_to_Software_Engineering/Architecture/Design_Patterns#Definition_of_a_Design_Pattern)
- [Status Cake - Useful patterns](https://www.statuscake.com/blog/useful-design-patterns/)