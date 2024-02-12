struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // Drop trait requires drop() method to implemented
    fn drop(&mut self) {
        println!("Droping CustomSmartPointer with data `{}`!", self.data);
    }
}
#[allow(dead_code)]
pub fn drop() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer Created");
    // first "d" will be droped and then "c", means in reverse order
    // order of printing
    // CustomSmartPointer Created
    // Droping CustomSmartPointer with data `other stuff`!
    // Droping CustomSmartPointer with data `my stuff`!

    /*
     * to manually clean the value we can "drop(d)" this drop function is different form our custom drop method
     * it is privided by rust standard library
     *
     * rust will clean memory automatically so we dont have to worry
     */
}
