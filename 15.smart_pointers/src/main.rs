/// when to us Box<T>

// 1. When you have a type whose size can’t be known at 
// compile time and you want to use a value of that type 
// in a context that requires an exact size

// 2. When you have a large amount of data and you want 
// to transfer ownership but ensure the data won’t be 
// copied when you do so

// 3. When you want to own a value and you care only that
// it’s a type that implements a particular trait rather 
// than being of a specific type

/// At compile time, Rust needs to know how much space a type takes up
// -> recursive type

use std::ops::Deref;
mod mock;

// https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html
fn deref_use() {
    let x = 5;
    let y = &x;

    assert_eq!(x, *y);

    // using Box<T> to reimplement
    // an instance of a box pointing to the value in x 
    // rather than a reference pointing to the value of x.
    let y_box = Box::new(x);
    assert_eq!(x, *y_box);

    let z_mybox = MyBox::new(x);
    // behind the scenes Rust actually ran this code: *(z_mybox.deref())
    // Note that the * operator is replaced with a call to the deref method 
    // and then a call to the * operator just once, each time we use a * in our code
    assert_eq!(5, *z_mybox);
}

// 2. create my Box(T)
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// treating a type like a reference by implement the `Deref` Trait
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// 3. `Drop` trait
// Specify the code to run when a value goes out of scope by implementing
// the Drop trait. The Drop trait requires you to implement one method named
// drop that takes a mutable reference to self.
#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // destructor
    fn drop(&mut self) {
        println!("Dropping with data `{}`!", self.data);
    }
}

// 4. The reference counted smart pointer
// To enable multiple ownership, Rust has a type called Rc<T>
// Note that Rc<T> is only for use in single-threaded scenarios
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>), // Box<List>
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

// Via immutable references, Rc<T> allows you to share data 
// between multiple parts of your program for reading only
fn use_rc_create_cons() {
    let strong_count = |a| Rc::strong_count(a); 

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", strong_count(&a));

    // The call to Rc::clone only increments the reference count, 
    // which doesn’t take much time. Deep copies of data can take a lot of time
    let b = Cons(3, Rc::clone(&a));
    // cloning an `Rc<T>` increases the reference count
    println!("count after creating b = {}", strong_count(&a));
    {
        // the implementation of the Drop trait decreases the 
        // reference count automatically when an Rc<T> value goes out of scope.
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", strong_count(&a));
    }

    println!("count after drop c = {}", strong_count(&a));
    println!("b = {:?}\n", b);
}

// 5. `RefCell<T>` and interior mutability pattern
/// Interior mutability is a design pattern in Rust 
/// that allows you to mutate data even when there 
/// are immutable references to that data
 
// About Borrowing rules:
/// a. At any given time, you can have either (but not both of) 
/// one mutable reference or any number of immutable references.
/// b. References must always be valid.

/// With references and Box<T>, the borrowing rules’ invariants 
/// are enforced at compile time. With RefCell<T>, these invariants 
/// are enforced at runtime. With references, if you break these 
/// rules, you’ll get a compiler error. With RefCell<T>, if 
/// you break these rules, your program will panic and exit.
/// Similar to Rc<T>, RefCell<T> is only for use in single-threaded 
/// scenarios and will give you a compile-time error if you try using 
/// it in a multithreaded context.

/// Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
/// a. Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
/// b. Box<T> allows immutable or mutable borrows checked at compile time; 
/// Rc<T> allows only immutable borrows checked at compile time; RefCell<T> 
/// allows immutable or mutable borrows checked at runtime.
/// c. Because RefCell<T> allows mutable borrows checked at runtime, 
/// you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
 
// Mutating the value inside an immutable value is the interior mutability pattern.
#[allow(dead_code)] 
fn interior_mutability() {
    let _x = 5;
    // !! cannot borrow immutable local variable `x` as mutable
    // let y = &mut x; 

}

/// Having multiple owners of mutable data by combining Rc<T> and RefCell<T>
/// A common way to use RefCell<T> is in combination with Rc<T>. Recall that 
/// Rc<T> lets you have multiple owners of some data, but it only gives 
/// immutable access to that data. If you have an Rc<T> that holds a RefCell<T>,
///  you can get a value that can have multiple owners and that you can mutate!
#[derive(Debug)]
enum MutList {
    MutCons(Rc<RefCell<i32>>, Rc<MutList>),
    Nil
}

use crate::MutList::{MutCons, Nil as Null};
use std::cell::RefCell;

fn multi_owners_mutable_data() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(MutCons(Rc::clone(&value), Rc::new(Null)));

    let b = MutCons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = MutCons(Rc::new(RefCell::new(7)), Rc::clone(&a));

    // The `borrow_mut` method returns a RefMut<T> smart pointer, 
    // and we use the dereference operator on it and change the inner value.
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    println!("\n")
}

/// 6. Reference cycles can leak memory
mod reference;
use reference::cycle_reference::CycList::{ Cons as CycleCons, Nil as CycleNil };

fn cycle_reference() {
    let strong_count = |a| Rc::strong_count(a);

    let a = Rc::new(CycleCons(5, RefCell::new(Rc::new(CycleNil))));
    println!("a initial rc count = {}", strong_count(&a));
    println!("a next item: {:?}", a.tail());

    let b = Rc::new(CycleCons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", strong_count(&a));
    println!("b initial rc count = {}", strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", strong_count(&b));
    println!("a rc count after changing a = {}", strong_count(&a));
    println!("\n");

    // trigger panic due to cycle reference
    // !!thread 'main' has overflowed its stack
    // println!("a next item = {:?}", a.tail());
}

fn main() {
    // 2. dereference
    deref_use();

    // 3. drop trait
    let a = CustomSmartPointer{ data: String::from("my stuff") };
    let b = CustomSmartPointer{ data: String::from("other stuff") };

    // the ownership system that makes sure references are always 
    // valid also ensures that drop gets called only once when the
    // value is no longer being used.
    // use std::mem::drop
    drop(a);
    println!("CustomSmartPointer created {:?}", b);

    // 4. Rc<T>
    println!("## Rc<T> multiple reference");
    use_rc_create_cons();

    // 5. RefCell<T> and Rc<T>
    println!("## multi owners mutable data");
    multi_owners_mutable_data();

    // 6. Cycle reference
    println!("## cycle reference");
    cycle_reference();
}
