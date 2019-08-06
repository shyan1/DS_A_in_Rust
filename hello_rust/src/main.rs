use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("Hello, world!");

    no_ref_counter();
    threading();
    channels();
    shared_state();
}

#[derive(Debug)]
struct Filename {
    name: String,
    ext: String,
}

fn no_ref_counter() {
    let name = String::from("main");
    let ext = String::from("rs");

    println!("{:?}", Filename { name, ext }); // name and ext moved here.

    // for _ in 0..3 {
    //     println!("{:?}", Filename { name: name.clone(), ext: ext.clone(), });
    // }
}

fn threading() {
    let x = String::from("hello");

    let handle = thread::spawn(move || {
        println!("{}", x);
    });

    handle.join().unwrap();
}

fn channels() {
    const N: i32 = 10;
    let (sender, receiver): (Sender<i32>, Receiver<i32>) = channel();

    let handles = (0..N).map(|i| {
        let _sender = sender.clone();
        thread::spawn(move || {
            let _ = _sender.send(i).unwrap();
        })
    });

    // iterators are lazy and for loop is not.
    // for h in handles {
    //     h.join().unwrap();
    // }
    handles.for_each(|h| h.join().unwrap());

    let res: Vec<i32> = (0..N).map(|_| receiver.recv().unwrap()).collect();

    println!("{:#?}", res);
}

fn shared_state() {
    let v = Arc::new(Mutex::new(vec![]));
    let handles = (0..10).map(|i| {
        let numbers = Arc::clone(&v);
        thread::spawn(move || {
            let mut vector = numbers.lock().unwrap();
            (*vector).push(i);
        })
    });

    handles.for_each(|h| {
        h.join().unwrap();
    });

    println!("{:?}", *v.lock().unwrap());
}
