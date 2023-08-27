
## Ownership:

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.


`Copy`: variables that use it do not move, but are copied


## Memory:

Programms involve 4 memory locations:
- `code/text`:region that stores the code that needs to be executed
- `static/ global`: static or global variables (`const`) that live the entire duration of the program
-  `stack`: function call variables are stored on the stack frame. Everything on the stack has to have a fixed size. So in order to be able to work with resizeable datastructures, the reference (pointer) to that structure is stored on the stack. The reference has a fixed size. So the variable on the stack is is either a primitive value or a reference to heap allocated data structures. 
- `heap`: memory location that is used for data that needs to outlive a function and/or for which the size is not known at compile time.

Putting things on the heap in Rust:
```rust
let x = Box::new(5); // Heap allocated
let y = 5; // Stack allocated
```


Primitives passed into a Rust function call are copied to the new stack frame. This is pass by value.

Heap allocated data passed into a Rust function will 'move'. If the function ends, the heap allocated data will be destroyed as it moves out of scope (given the function does not return it).

In order to prevent from moving the data into the function, we can pass in a reference. If we pass a reference into a function, the function will 'borrow' the heap allocated data. When the function ends, the reference goes out of scope and the reference is destroyed. The heap allocated data will remain.

We cannot alter anything borrowed. If we want to change the heap allocated data inside the function call without moving the data into the function, we must pass in a mutable reference. This will allow us to alter the heap allocated data inside the function without moving it there. For any given value, we can only hand out 1 mutable borrow at a time.