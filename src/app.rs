use std::rc::Rc;
use std::sync::{Arc};
use lazy_static::lazy_static;
use dominator::{Dom, class, html};

use crate::parent::{Parent};

pub struct App {
    parent: Arc<Parent>,
}

impl App {

    pub fn new() -> Rc<Self> {

        Rc::new(Self {
            parent: Parent::new(),
        })
    }

    pub fn render(app: Rc<Self>) -> Dom {
        
        lazy_static! {
            static ref ROOT_CLASS: String = class! {
                .style("overflow-x", "hidden")
                .style("color", "red")
            };
        }
        
        html!("div", {
            .class(&*ROOT_CLASS)
            .children(&mut [
                Parent::render(app.parent.clone()),
            ])
        })
    }
}
