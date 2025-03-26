use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;
use std::sync::mpsc;

#[derive(Clone)]
pub struct Clock {
    tick_duration: Duration,
    tick_count: Arc<Mutex<u32>>,
    subscribers: Arc<Mutex<Vec<mpsc::Sender<u32>>>>,
}

impl Clock {
    // Create a new Clock
    pub fn new(tick_duration: Duration) -> Self {
        let clock = Clock {
            tick_duration,
            tick_count: Arc::new(Mutex::new(0)),
            subscribers: Arc::new(Mutex::new(Vec::new())),
        };

        // Spawn a background thread to handle the ticking
        let tick_count = clock.tick_count.clone();
        let subscribers = clock.subscribers.clone();
        thread::spawn(move || {
            loop {
                thread::sleep(tick_duration);

                let mut count = tick_count.lock().unwrap();
                *count += 1;

                // send to all subscribers
                let subscribers = subscribers.lock().unwrap();
                for subscriber in subscribers.iter() {
                    let _ = subscriber.send(*count);
                }
            }
        });

        clock
    }

    pub fn channel(&self) -> ClockChannel {
        let (tx, rx) = mpsc::channel();

        // don't forget to add it into the list!
        self.subscribers.lock().unwrap().push(tx);

        ClockChannel { rx }
    }
}

pub struct ClockChannel {
    rx: mpsc::Receiver<u32>,
}

impl ClockChannel {
    pub fn next(&self) -> u32 {
        self.rx.recv().unwrap()
    }
}

pub struct Ticker<F> {
    action: Arc<Mutex<F>>,
}

impl<F> Ticker<F>
where
    F: FnMut() + Send + 'static,
{
    pub fn new(clock: &Clock, action: F) -> Self {
        let action = Arc::new(Mutex::new(action)); 

        let clock_channel = clock.channel(); 

        // a running thread
        thread::spawn({
            let action = action.clone(); // needs the clone to be able to make it into the thread
            move || {
                loop {
                    let _tick = clock_channel.next(); 

                    // call the function
                    let mut action = action.lock().unwrap();  
                    action(); 
                }
            }
        });

        // could be done without returns???
        Ticker { action }
    }
}

pub struct Alarm {
    handle: Option<thread::JoinHandle<()>>,
}

impl Alarm {
    pub fn new(clock: &Clock, ticks_to_wait: u32, action: impl FnOnce() + Send + 'static) -> Self {
        let clock_channel = clock.channel(); 

        let handle = thread::spawn({
            move || {
                let mut tick_count = 0;
                while tick_count < ticks_to_wait {
                    tick_count = clock_channel.next(); 
                }
                // not keeping the action as it should just run once and die
                action(); 
            }
        });

        // again? really necessary?
        Alarm {
            handle: Some(handle),
        }
    }
}

fn main() {
    let clock = Clock::new(Duration::from_secs(1));

    Alarm::new(&clock, 2, || println!("Alarm1!"));
    Alarm::new(&clock, 4, || println!("Alarm2!"));
    Alarm::new(&clock, 6, || println!("Alarm3!"));

    Alarm::new(&clock.clone(), 4, move || {
            println!("FIRST STAGE");
            Alarm::new(&clock, 4, move || println!("Four!"));
        });

    thread::sleep(Duration::from_secs(10));
}
