// Copyright 2020 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Additional unit tests that cross file or module boundaries.

mod harness;
mod helpers;
mod layout_tests;

use std::cell::Cell;
use std::rc::Rc;

use crate::widget::*;
use crate::*;
use harness::*;
use helpers::*;

/// test that the first widget to request focus during an event gets it.
#[test]
fn propogate_hot() {
    let (button, pad, root, empty) = widget_id4();

    let root_rec = Recording::default();
    let padding_rec = Recording::default();
    let button_rec = Recording::default();

    let widget = Split::vertical(
        SizedBox::empty().with_id(empty),
        Button::new("hot", |_, _, _| {})
            .record(&button_rec)
            .with_id(button)
            .padding(50.)
            .record(&padding_rec)
            .with_id(pad),
    )
    .record(&root_rec)
    .with_id(root);

    fn make_mouse(x: f64, y: f64) -> MouseEvent {
        let pos = Point::new(x, y);
        MouseEvent {
            pos,
            window_pos: pos,
            mods: KeyModifiers::default(),
            count: 0,
            button: MouseButton::Left,
        }
    }
    Harness::create((), widget, |harness| {
        harness.send_initial_events();
        harness.just_layout();

        // we don't care about setup events, so discard them now.
        root_rec.clear();
        padding_rec.clear();
        button_rec.clear();

        harness.inspect_state(|state| assert!(!state.is_hot));

        // What we are doing here is moving the mouse to different widgets,
        // and verifying both the widget's `is_hot` status and also that
        // each widget received the expected HotChanged messages.

        harness.event(Event::MouseMoved(make_mouse(10., 10.)));
        assert!(harness.get_state(root).is_hot);
        assert!(harness.get_state(empty).is_hot);
        assert!(!harness.get_state(pad).is_hot);

        assert_matches!(root_rec.next(), Record::L(LifeCycle::HotChanged(true)));
        assert_matches!(root_rec.next(), Record::E(Event::MouseMoved(_)));
        assert!(root_rec.is_empty() && padding_rec.is_empty() && button_rec.is_empty());

        harness.event(Event::MouseMoved(make_mouse(210., 10.)));

        assert!(harness.get_state(root).is_hot);
        assert!(!harness.get_state(empty).is_hot);
        assert!(!harness.get_state(button).is_hot);
        assert!(harness.get_state(pad).is_hot);

        assert_matches!(root_rec.next(), Record::E(Event::MouseMoved(_)));
        assert_matches!(padding_rec.next(), Record::L(LifeCycle::HotChanged(true)));
        assert_matches!(padding_rec.next(), Record::E(Event::MouseMoved(_)));
        assert!(root_rec.is_empty() && padding_rec.is_empty() && button_rec.is_empty());

        harness.event(Event::MouseMoved(make_mouse(260., 60.)));
        assert!(harness.get_state(root).is_hot);
        assert!(!harness.get_state(empty).is_hot);
        assert!(harness.get_state(button).is_hot);
        assert!(harness.get_state(pad).is_hot);

        assert_matches!(root_rec.next(), Record::E(Event::MouseMoved(_)));
        assert_matches!(padding_rec.next(), Record::E(Event::MouseMoved(_)));
        assert_matches!(button_rec.next(), Record::L(LifeCycle::HotChanged(true)));
        assert_matches!(button_rec.next(), Record::E(Event::MouseMoved(_)));
        assert!(root_rec.is_empty() && padding_rec.is_empty() && button_rec.is_empty());

        harness.event(Event::MouseMoved(make_mouse(10., 10.)));
        assert!(harness.get_state(root).is_hot);
        assert!(harness.get_state(empty).is_hot);
        assert!(!harness.get_state(button).is_hot);
        assert!(!harness.get_state(pad).is_hot);

        assert_matches!(root_rec.next(), Record::E(Event::MouseMoved(_)));
        assert_matches!(padding_rec.next(), Record::L(LifeCycle::HotChanged(false)));
        assert_matches!(padding_rec.next(), Record::E(Event::MouseMoved(_)));
        assert_matches!(button_rec.next(), Record::L(LifeCycle::HotChanged(false)));
        assert_matches!(button_rec.next(), Record::E(Event::MouseMoved(_)));
        assert!(root_rec.is_empty() && padding_rec.is_empty() && button_rec.is_empty());
    });
}
#[test]
fn take_focus() {
    const TAKE_FOCUS: Selector = Selector::new("druid-tests.take-focus");

    /// A widget that takes focus when sent a particular command.
    /// The widget records focus change events into the inner cell.
    fn make_focus_taker(inner: Rc<Cell<Option<bool>>>) -> impl Widget<bool> {
        ModularWidget::new(inner)
            .event_fn(|_, ctx, event, _data, _env| {
                if let Event::Command(cmd) = event {
                    if cmd.selector == TAKE_FOCUS {
                        ctx.request_focus();
                    }
                }
            })
            .lifecycle_fn(|is_focused, _, event, _data, _env| {
                if let LifeCycle::FocusChanged(focus) = event {
                    is_focused.set(Some(*focus));
                }
            })
    }

    let (id_1, id_2, _id_3) = widget_id3();

    // we use these so that we can check the widget's internal state
    let left_focus: Rc<Cell<Option<bool>>> = Default::default();
    let right_focus: Rc<Cell<Option<bool>>> = Default::default();
    assert!(left_focus.get().is_none());

    let left = make_focus_taker(left_focus.clone()).with_id(id_1);
    let right = make_focus_taker(right_focus.clone()).with_id(id_2);
    let app = Split::vertical(left, right).padding(5.0);
    let data = true;

    Harness::create(data, app, |harness| {
        harness.send_initial_events();
        // nobody should have focus
        assert!(left_focus.get().is_none());
        assert!(right_focus.get().is_none());

        // this is sent to all widgets; the first widget to request focus should get it
        harness.submit_command(TAKE_FOCUS, None);
        assert_eq!(harness.window().focus, Some(id_1));
        assert_eq!(left_focus.get(), Some(true));
        assert_eq!(right_focus.get(), None);

        // this is sent to a specific widget; it should get focus
        harness.submit_command(TAKE_FOCUS, id_2);
        assert_eq!(harness.window().focus, Some(id_2));
        assert_eq!(left_focus.get(), Some(false));
        assert_eq!(right_focus.get(), Some(true));
    })
}

#[test]
fn simple_lifecyle() {
    let record = Recording::default();
    let widget = SizedBox::empty().record(&record);
    Harness::create(true, widget, |harness| {
        harness.send_initial_events();
        assert_matches!(record.next(), Record::L(LifeCycle::WidgetAdded));
        assert_matches!(record.next(), Record::E(Event::WindowConnected));
        assert_matches!(record.next(), Record::E(Event::Size(_)));
        assert!(record.is_empty());
    })
}

#[test]
/// Test that lifecycle events are sent correctly to a child added during event
/// handling
fn adding_child_lifecycle() {
    let record = Recording::default();
    let record_new_child = Recording::default();
    let record_new_child2 = record_new_child.clone();

    let replacer = ReplaceChild::new(TextBox::new(), move || {
        Split::vertical(TextBox::new(), TextBox::new().record(&record_new_child2))
    });

    let widget = Split::vertical(Label::new("hi").record(&record), replacer);

    Harness::create(String::new(), widget, |harness| {
        harness.send_initial_events();

        assert_matches!(record.next(), Record::L(LifeCycle::WidgetAdded));
        assert_matches!(record.next(), Record::E(Event::WindowConnected));
        assert!(record.is_empty());

        assert!(record_new_child.is_empty());

        harness.submit_command(REPLACE_CHILD, None);

        assert_matches!(record.next(), Record::E(Event::Command(_)));

        assert_matches!(record_new_child.next(), Record::L(LifeCycle::WidgetAdded));
        assert!(record_new_child.is_empty());
    })
}

#[test]
fn participate_in_autofocus() {
    let (id_1, id_2, id_3, id_4, id_5, id_6) = widget_id6();

    // this widget starts with a single child, and will replace them with a split
    // when we send it a command.
    let replacer = ReplaceChild::new(TextBox::new().with_id(id_4), move || {
        Split::vertical(TextBox::new().with_id(id_5), TextBox::new().with_id(id_6))
    });

    let widget = Split::vertical(
        Flex::row()
            .with_child(TextBox::new().with_id(id_1), 1.0)
            .with_child(TextBox::new().with_id(id_2), 1.0)
            .with_child(TextBox::new().with_id(id_3), 1.0),
        replacer,
    );

    Harness::create("my test text".to_string(), widget, |harness| {
        // verify that all widgets are marked as having children_changed
        // (this should always be true for a new widget)
        harness.inspect_state(|state| assert!(state.children_changed));

        harness.send_initial_events();
        // verify that we start out with four widgets registered for focus
        assert_eq!(harness.window().focus_chain(), &[id_1, id_2, id_3, id_4]);

        // tell the replacer widget to swap its children
        harness.submit_command(REPLACE_CHILD, None);

        // verify that the two new children are registered for focus.
        assert_eq!(
            harness.window().focus_chain(),
            &[id_1, id_2, id_3, id_5, id_6]
        );

        // verify that no widgets still report that their children changed:
        harness.inspect_state(|state| assert!(!state.children_changed))
    })
}

#[test]
fn child_tracking() {
    let (id_1, id_2, id_3, id_4) = widget_id4();

    let widget = Split::vertical(
        SizedBox::empty().with_id(id_1),
        SizedBox::empty().with_id(id_2),
    )
    .with_id(id_3)
    .padding(5.0)
    .with_id(id_4);

    Harness::create(true, widget, |harness| {
        harness.send_initial_events();
        let root = harness.get_state(id_4);
        assert_eq!(root.children.entry_count(), 3);
        assert!(root.children.contains(&id_1));
        assert!(root.children.contains(&id_2));
        assert!(root.children.contains(&id_3));

        let split = harness.get_state(id_3);
        assert!(split.children.contains(&id_1));
        assert!(split.children.contains(&id_2));
        assert_eq!(split.children.entry_count(), 2);
    });
}

#[test]
/// Test that all children are registered correctly after a child is replaced.
fn register_after_adding_child() {
    let (id_1, id_2, id_3, id_4, id_5, id_6) = widget_id6();
    let id_7 = WidgetId::next();

    let replacer = ReplaceChild::new(TextBox::new().with_id(id_1), move || {
        Split::vertical(TextBox::new().with_id(id_2), TextBox::new().with_id(id_3)).with_id(id_7)
    })
    .with_id(id_6);

    let widget = Split::vertical(Label::new("hi").with_id(id_4), replacer).with_id(id_5);

    Harness::create(String::new(), widget, |harness| {
        harness.send_initial_events();

        assert!(harness.get_state(id_5).children.contains(&id_6));
        assert!(harness.get_state(id_5).children.contains(&id_1));
        assert!(harness.get_state(id_5).children.contains(&id_4));
        assert_eq!(harness.get_state(id_5).children.entry_count(), 3);

        harness.submit_command(REPLACE_CHILD, None);

        assert!(harness.get_state(id_5).children.contains(&id_6));
        assert!(harness.get_state(id_5).children.contains(&id_4));
        assert!(harness.get_state(id_5).children.contains(&id_7));
        assert!(harness.get_state(id_5).children.contains(&id_2));
        assert!(harness.get_state(id_5).children.contains(&id_3));
        assert_eq!(harness.get_state(id_5).children.entry_count(), 5);
    })
}
