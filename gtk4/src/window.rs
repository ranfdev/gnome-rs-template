use crate::config::{APP_ID, PROFILE};
use glib::signal::Inhibit;
use gtk::subclass::prelude::*;
use gtk::{self, prelude::*};
use gtk::{gio, glib, CompositeTemplate};

use crate::application::ExampleApplication;

mod imp {
    use super::*;
    use glib::subclass;

    #[derive(Debug, CompositeTemplate)]
    pub struct ExampleApplicationWindow {
        #[template_child]
        pub headerbar: TemplateChild<gtk::HeaderBar>,
        pub settings: gio::Settings,
    }

    impl ObjectSubclass for ExampleApplicationWindow {
        const NAME: &'static str = "ExampleApplicationWindow";
        type Type = super::ExampleApplicationWindow;
        type ParentType = gtk::ApplicationWindow;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib::object_subclass!();

        fn new() -> Self {
            Self {
                headerbar: TemplateChild::default(),
                settings: gio::Settings::new(APP_ID),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.set_template_from_resource("/com/belmoussaoui/GtkRustTemplate/window.ui");
            Self::bind_template_children(klass);
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self::Type>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ExampleApplicationWindow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            let builder =
                gtk::Builder::from_resource("/com/belmoussaoui/GtkRustTemplate/shortcuts.ui");
            let shortcuts = builder.get_object("shortcuts").unwrap();
            obj.set_help_overlay(Some(&shortcuts));

            // Devel Profile
            if PROFILE == "Devel" {
                obj.get_style_context().add_class("devel");
            }

            // load latest window state
            obj.load_window_size();
        }
    }

    impl WindowImpl for ExampleApplicationWindow {
        // save window state on delete event
        fn close_request(&self, obj: &Self::Type) -> Inhibit {
            if let Err(err) = obj.save_window_size() {
                log::warn!("Failed to save window state, {}", &err);
            }
            Inhibit(false)
        }
    }

    impl WidgetImpl for ExampleApplicationWindow {}
    impl ApplicationWindowImpl for ExampleApplicationWindow {}
}

glib::wrapper! {
    pub struct ExampleApplicationWindow(ObjectSubclass<imp::ExampleApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

impl ExampleApplicationWindow {
    pub fn new(app: &ExampleApplication) -> Self {
        let window: Self =
            glib::Object::new(&[]).expect("Failed to create ExampleApplicationWindow");
        window.set_application(Some(app));

        // Set icons for shell
        gtk::Window::set_default_icon_name(APP_ID);

        window
    }

    pub fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = &imp::ExampleApplicationWindow::from_instance(self).settings;

        let size = self.get_default_size();

        settings.set_int("window-width", size.0)?;
        settings.set_int("window-height", size.1)?;

        settings.set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let settings = &imp::ExampleApplicationWindow::from_instance(self).settings;

        let width = settings.get_int("window-width");
        let height = settings.get_int("window-height");
        let is_maximized = settings.get_boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }
}