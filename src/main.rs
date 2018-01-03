extern crate uuid;

mod actors;

use std::sync::{Mutex, Arc};
// use std::time::Duration;
use std::any::Any;

use actors::{Actor, ActorSystem, Context};

struct SomeActor {
    state: Mutex<i32>,
}

impl SomeActor {
    fn set_state<F>(&self, f: F)
    where
        F: Fn(i32) -> i32,
    {
        let mut state = self.state.lock().unwrap();
        (*state) = f(*state);
    }
}

impl Actor for SomeActor {
    fn new() -> Arc<SomeActor> {
        let state = Mutex::new(0);
        Arc::new(SomeActor { state })
    }

    fn receive(&self, message: Box<Any>, context: Context) -> Option<Vec<Box<Any>>> {
        println!("My pid is: {}", context.sender.pid);
        // let our_actor = context.system.spawn(SomeActor::new());
        // for i in 0..100 {
        //     context.system.tell(&our_actor, MyMessage::Some(i));
        // }

        // downcast message back to our type
        let msg = message.downcast::<MyMessage>().unwrap();
        match *msg {
            MyMessage::Some(value) => self.set_state(|v| v + value as i32),
        }
        let state = self.state.lock().unwrap();
        println!("I've received some message and my state is: {:?}", *state);
        None
    }
}

#[derive(Clone, Debug)]
enum MyMessage {
    Some(u8),
}

fn main() {
    let system = ActorSystem::new(8);
    let our_actor = system.spawn(SomeActor::new());
    for i in 0..10 {
        system.tell(&our_actor, MyMessage::Some(i));
    }

    // for now we want to block everything
    while system.is_alive() {}
    // after implementing system.shoutdown we may not need global_queue at all;
    // kill all threads
    // for _ in 0..NUM_OF_THREADS {
    //     system.threads_queue.send(ThreadMessage::Die);
    //     println!("Pushed death order")
    // }

    // // wait for all threads death
    // while system.alive_threads > 0 {
    //     let head = system.global_queue.recv();
    //     match head {
    //         Ok(msg) => {
    //             match msg {
    //                 SystemMessage::Died(id) => {
    //                     println!("Thread {} died!", id);
    //                     system.alive_threads -= 1;
    //                     println!("Alive threads: {}", system.alive_threads);
    //                 }
    //             }
    //         }
    //         Err(_) => println!("Head is empty"),

    //     }
    // }
}
