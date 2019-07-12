# Gtk + Rust + Meson + Flatpak = <3

A boilerplate template to get started with GTK, Rust, Meson, Flatpak made for GNOME. It can be adapted for other desktop environements like elementary.

## What does it contains?
    - A simple window with a headerbar
    - Bunch of useful files that you SHOULD ship with your application on Linux:
        - Appdata: describe your application for the different application stores out there;
        - Desktop: the application launcher;
        - Icons: this repo contains two icons a normal & monochromatic icon (symbolic) per the GNOME HIG;
    - Two Flatpak manifests: a stable & nightly one
    - Dual installation support
    - Uses Meson for building the application
    - Bundles the UI files & the CSS using gresources
    - A pre-commit hook to run rustfmt on your code
    - Tests to validate your Appdata & Desktop files
    - Gsettings to store the window state, more settings could be added
    - Gitlab CI to produce flatpak nightlies
    - i18n support