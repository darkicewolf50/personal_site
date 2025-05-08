//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.
//!
//!
//! The [`Home`] and [`Blog`] components will be rendered when the current route is [`Route::Home`] or [`Route::Blog`] respectively.
//!
//!
//! The [`Navbar`] component will be rendered on all pages of our app since every page is under the layout. The layout defines
//! a common wrapper around all child routes.

mod new_home;
pub use new_home::NewHome;

mod blog;
pub use blog::{Blog, Blogs};

mod navbar;
pub use navbar::Navbar;

mod home;
pub use home::Home;

mod contact;
pub use contact::Contact;

mod projects;
pub use projects::Projects;

mod contact_me;
pub use contact_me::ContactMe;
