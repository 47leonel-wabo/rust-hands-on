use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    /*
        Reference (&) is pointer, but it only borrows data
        While pointer owns data
        Common smart pointer in standard library
        Box<T>
        Rc<T>
        Ref<T> and RefMut<T>
     */

    /*
        Box
        Common and simple smart pointer
     */
    // Box<T>, its allows to store data on heap instead on stack
    let x = 17;
    let smt_b = Box::new(x);
    println!("Pointer value = {}", smt_b);
    println!("Dereference value = {}", *smt_b); // Dereference with '*'

    // Deref, allow to customize usage of '*' for Box
    let y = 1010;
    let custom_smt_b = CustomBox::new(y);
    println!("Pointer value = {:#?}", custom_smt_b);
    println!("Dereference value = {}", *custom_smt_b);

    /*
        Rc<T>
        Reference counted, to enable multiple ownership
        it keeps track the number of references to a value and determines whether
        that value still in use or not.
     */
    let ms = MyStruct { _value: 10 }; // Underscore "_" is used to avoid warnings

    let a = Rc::new(ms);
    println!("Reference count: {}", Rc::strong_count(&a)); // 1

    let b = Rc::clone(&a);
    println!("Reference count: {}", Rc::strong_count(&a)); // 2

    {
        let _c = Rc::clone(&b);
        let _d = Rc::clone(&a);
        println!("Reference count: {}", Rc::strong_count(&a)); // 4
    } // Out of this scope, reference is loosed

    println!("Reference count: {}", Rc::strong_count(&a)); // 2

    /*
        RefCell<T>
        it is used to achieve "interior mutability pattern"
        it represents single ownership over the data that it holds
     */
    let var = 123;
    /*
        // The problem is:
        var = 45; // ERROR
        let val = &mut var; // ERROR too
     */
    // Solution
    let v_cell = RefCell::new(var);
    let mut vb = v_cell.borrow_mut();
    *vb = 50;
    println!("vb = {}", vb);

} // Drop will be called here

#[derive(Debug)] // Avoid error for un-implemented methods
struct CustomBox<T>(T); // Generic struct definition

// Generic struct implementation
impl<T> CustomBox<T> {
    fn new(x: T) -> CustomBox<T> {
        CustomBox(x)
    }
}

// Will be called when our pointer is dereference
impl<T> Deref for CustomBox<T> {
    type Target = T;
    fn deref(&self) -> &T { // method override
        println!("Calling Deref...");
        &self.0
    }
}

// Will be called when our pointer goes out of scope
impl<T> Drop for CustomBox<T> {
    fn drop(&mut self) { // method override
        println!("Pointer out of scope, dropping...");
    }
}

struct MyStruct {
    _value: i32,
}