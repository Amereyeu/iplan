use adw;
use adw::subclass::prelude::*;
use adw::traits::PreferencesRowExt;
use glib::{once_cell::sync::Lazy, subclass::Signal};
use gtk::{glib, glib::Properties, prelude::*};
use rusqlite;
use std::cell::RefCell;

use crate::db::models::{Reminder, Task};
use crate::db::operations::read_reminder;
use crate::views::reminder::ReminderWindow;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate, Properties)]
    #[template(resource = "/ir/imansalmani/iplan/ui/reminder/reminder_row.ui")]
    #[properties(wrapper_type=super::ReminderRow)]
    pub struct ReminderRow {
        #[property(get, set)]
        pub reminder: RefCell<Reminder>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ReminderRow {
        const NAME: &'static str = "ReminderRow";
        type Type = super::ReminderRow;
        type ParentType = adw::ActionRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ReminderRow {
        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![
                    Signal::builder("changed")
                        .param_types([Reminder::static_type()])
                        .build(),
                    Signal::builder("removed")
                        .param_types([i64::static_type()])
                        .build(),
                ]
            });
            SIGNALS.as_ref()
        }

        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }
    }
    impl WidgetImpl for ReminderRow {}
    impl ListBoxRowImpl for ReminderRow {}
    impl PreferencesRowImpl for ReminderRow {}
    impl ActionRowImpl for ReminderRow {}
}

glib::wrapper! {
    pub struct ReminderRow(ObjectSubclass<imp::ReminderRow>)
        @extends gtk::Widget, gtk::ListBoxRow, adw::PreferencesRow, adw::ActionRow,
        @implements gtk::Buildable;
}

#[gtk::template_callbacks]
impl ReminderRow {
    pub fn new(reminder: Reminder) -> Self {
        let obj: Self = glib::Object::builder()
            .property("reminder", reminder)
            .build();
        obj.set_labels();
        obj
    }

    fn set_labels(&self) {
        let reminder = self.reminder();
        let datetime = reminder.datetime_datetime();
        self.set_title(&format!(
            "{} {}",
            Task::date_display(&datetime),
            datetime.format("%H:%M").unwrap()
        ));
    }

    #[template_callback]
    fn handle_activated(&self) {
        let win = self.root().and_downcast::<gtk::Window>().unwrap();
        let modal = ReminderWindow::new(&win.application().unwrap(), &win, self.reminder(), true);
        modal.present();
        modal.connect_close_request(
            glib::clone!(@weak self as obj => @default-return gtk::Inhibit(false), move |_| {
                match read_reminder(obj.reminder().id()) {
                    Ok(reminder) => {
                        obj.emit_by_name::<()>("changed", &[&reminder]);
                        obj.set_reminder(reminder);
                        obj.set_labels();
                    }
                    Err(err) => match err {
                        rusqlite::Error::QueryReturnedNoRows  => {
                            let reminders_box = obj.parent().and_downcast::<gtk::ListBox>().unwrap();
                            obj.emit_by_name::<()>("removed", &[&obj.reminder().id()]);
                            reminders_box.remove(&obj);
                        },
                        err => panic!("{err}")
                    }
                }
                gtk::Inhibit(false)
            }),
        );
    }
}
