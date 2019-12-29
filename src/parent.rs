use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use core::marker::PhantomData;
use dominator::{Dom, class, html, clone, events};

use futures_signals::signal::{Mutable, SignalExt};

#[derive(Debug)]
pub struct EventDispatcher<A, F> {
    listener: Arc<Mutex<F>>,
    argument: PhantomData<A>,
}

impl<A, F> Clone for EventDispatcher<A, F> {
    #[inline]
    fn clone(&self) -> Self {
        EventDispatcher {
            listener: self.listener.clone(),
            argument: self.argument,
        }
    }
}

impl<A, F> EventDispatcher<A, F> where F: FnMut(A) {
    #[inline]
    pub fn new(listener: F) -> Self {
        Self {
            listener: Arc::new(Mutex::new(listener)),
            argument: PhantomData,
        }
    }

    #[inline]
    pub fn send(&self, event: A) {
        let listener = &mut *self.listener.lock().unwrap();
        listener(event);
    }

}

#[derive(Clone, Copy)]
pub enum Event {
    LeftClick,
    RightClick,
}

pub struct Child {
    
}

impl Child {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            
        })
    }

    pub fn render<F>(_: Arc<Self>, on_event: F) -> Dom where F: FnMut(Event) + 'static {
        lazy_static! {
            static ref ROOT_CLASS: String = class! {
                .style("display", "inline-block")
                .style("background-color", "blue")
                .style("padding", "10px")
                .style("width", "200px")
                .style("height", "200px")
            };
  
        }

        let e = EventDispatcher::new(on_event);

        html!("div", {
            .class(&*ROOT_CLASS)
            .event(clone!(e => move |_: events::Click| {
                e.send(Event::LeftClick);
            }))

            .event(clone!(e => move |_: events::ContextMenu| {
                e.send(Event::RightClick);
            }))
        })
    }
}


pub struct Parent {
    child: Arc<Child>,
    clicked: Mutable<i32>,
    right_clicked: Mutable<i32>,
}

impl Parent {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            child: Child::new(),
            clicked: Mutable::new(0),
            right_clicked: Mutable::new(0),
        })
    }

    pub fn render(parent: Arc<Self>) -> Dom {
        lazy_static! {
            static ref ROOT_CLASS: String = class! {
                .style("display", "inline-block")
                .style("background-color", "green")
                .style("padding", "10px")
                .style("width", "500px")
                .style("height", "500px")
            };
  
            static ref TEXT_CLASS: String = class! {
                .style("color", "white")
                .style("font-weight", "bold")
            };

        }

        html!("div", {
            .class(&*ROOT_CLASS)
            .children(&mut [

                html!("div", {
                    .class(&*TEXT_CLASS)
                    .text_signal(parent.clicked.signal().map(|x| format!("Clicked: {}", x)))
                }),

                html!("div", {
                    .class(&*TEXT_CLASS)
                    .text_signal(parent.right_clicked.signal().map(|x| format!("Right Clicked: {}", x)))
                }),

                Child::render(parent.child.clone(), move |event| {
                    match event {
                        Event::LeftClick => {
                            parent.clicked.replace_with(|x| *x + 1);
                        },
                        Event::RightClick => {
                            parent.right_clicked.replace_with(|x| *x + 1);
                        },
                    }
                }),
            ])
        })
    }
}