use adw::subclass::prelude::*;
use adw::traits::ActionRowExt;
use gettextrs::gettext;
use gtk::glib::{once_cell::sync::Lazy, subclass::*, Properties};
use gtk::{glib, prelude::*};
use std::cell::Cell;

const DATE_FORMAT: &str = "%B %e, %Y";

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate, Properties)]
    #[template(resource = "/ir/imansalmani/iplan/ui/date_row.ui")]
    #[properties(type_wrapper=super::DateRow)]
    pub struct DateRow {
        #[template_child]
        pub calendar: TemplateChild<gtk::Calendar>,
        #[template_child]
        pub menu_button: TemplateChild<gtk::MenuButton>,
        #[template_child]
        pub clear_button: TemplateChild<gtk::Button>,
        #[property(get, set)]
        pub clear_option: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DateRow {
        const NAME: &'static str = "DateRow";
        type Type = super::DateRow;
        type ParentType = adw::ActionRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DateRow {
        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("date-changed")
                    .param_types([glib::DateTime::static_type()])
                    .build()]
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
    impl WidgetImpl for DateRow {}
    impl ListBoxRowImpl for DateRow {}
    impl PreferencesRowImpl for DateRow {}
    impl ActionRowImpl for DateRow {}
}

glib::wrapper! {
    pub struct DateRow(ObjectSubclass<imp::DateRow>)
        @extends gtk::Widget, gtk::ListBoxRow, adw::PreferencesRow, adw::ActionRow,
        @implements gtk::Buildable, gtk::Actionable, gtk::Accessible, gtk::ConstraintTarget;
}

impl Default for DateRow {
    fn default() -> Self {
        glib::Object::new::<Self>()
    }
}

#[gtk::template_callbacks]
impl DateRow {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_datetime(&self, datetime: &glib::DateTime) {
        let imp = self.imp();
        imp.calendar.set_year(datetime.year());
        imp.calendar.set_month(datetime.month() - 1);
        imp.calendar.set_day(datetime.day_of_month());
        self.refresh_row(datetime);
        self.show_clear_button();
    }

    pub fn calculate_datetime(&self) -> glib::DateTime {
        let calendar: &gtk::Calendar = self.imp().calendar.as_ref();
        glib::DateTime::new(
            &glib::TimeZone::local(),
            calendar.year(),
            calendar.month() + 1,
            calendar.day(),
            0,
            0,
            0.0,
        )
        .unwrap()
    }

    fn refresh_row(&self, datetime: &glib::DateTime) {
        let now = glib::DateTime::now_local().unwrap().ymd();
        let subtitle = if now == datetime.ymd() {
            gettext("Today")
        } else {
            datetime.format(DATE_FORMAT).unwrap().to_string()
        };
        self.set_subtitle(&subtitle);
    }

    fn show_clear_button(&self) {
        if self.clear_option() {
            self.imp().clear_button.set_visible(true);
        }
    }

    #[template_callback]
    fn handle_clear_clicked(&self, clear_button: gtk::Button) {
        clear_button.set_visible(false);
        self.imp().calendar.clear_marks();
        self.set_subtitle("");
        self.emit_by_name::<()>(
            "date-changed",
            &[&glib::DateTime::from_unix_local(0).unwrap()],
        );
    }

    #[template_callback]
    fn handle_day_selected(&self, _calendar: gtk::Calendar) {
        let imp = self.imp();
        imp.menu_button.popdown();
        self.show_clear_button();
        let datetime = self.calculate_datetime();
        self.refresh_row(&datetime);
        self.emit_by_name::<()>("date-changed", &[&datetime]);
    }
}
