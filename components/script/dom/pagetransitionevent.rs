/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use crate::dom::bindings::codegen::Bindings::PageTransitionEventBinding;
use crate::dom::bindings::codegen::Bindings::PageTransitionEventBinding::PageTransitionEventMethods;
use crate::dom::bindings::error::Fallible;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::reflect_dom_object;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::event::Event;
use crate::dom::window::Window;
use dom_struct::dom_struct;
use servo_atoms::Atom;
use std::cell::Cell;

// https://html.spec.whatwg.org/multipage/#pagetransitionevent
#[dom_struct]
pub struct PageTransitionEvent {
    event: Event,
    persisted: Cell<bool>,
}

impl PageTransitionEvent {
    fn new_inherited() -> PageTransitionEvent {
        PageTransitionEvent {
            event: Event::new_inherited(),
            persisted: Cell::new(false),
        }
    }

    pub fn new_uninitialized(window: &Window) -> DomRoot<PageTransitionEvent> {
        reflect_dom_object(
            Box::new(PageTransitionEvent::new_inherited()),
            window,
            PageTransitionEventBinding::Wrap,
        )
    }

    pub fn new(
        window: &Window,
        type_: Atom,
        bubbles: bool,
        cancelable: bool,
        persisted: bool,
    ) -> DomRoot<PageTransitionEvent> {
        let ev = PageTransitionEvent::new_uninitialized(window);
        ev.persisted.set(persisted);
        {
            let event = ev.upcast::<Event>();
            event.init_event(type_, bubbles, cancelable);
        }
        ev
    }

    #[allow(non_snake_case)]
    pub fn Constructor(
        window: &Window,
        type_: DOMString,
        init: &PageTransitionEventBinding::PageTransitionEventInit,
    ) -> Fallible<DomRoot<PageTransitionEvent>> {
        Ok(PageTransitionEvent::new(
            window,
            Atom::from(type_),
            init.parent.bubbles,
            init.parent.cancelable,
            init.persisted,
        ))
    }
}

impl PageTransitionEventMethods for PageTransitionEvent {
    // https://html.spec.whatwg.org/multipage/#dom-pagetransitionevent-persisted
    fn Persisted(&self) -> bool {
        self.persisted.get()
    }

    // https://dom.spec.whatwg.org/#dom-event-istrusted
    fn IsTrusted(&self) -> bool {
        self.event.IsTrusted()
    }
}
