//! A [Netscape Bookmark File format] parser.
//!
//! The name is a subtile *mélange* between *bookmark* and *book mart*. It also works as a mashup with the english *book* and the german *markt*.
//!
//! [Netscape Bookmark File format]: https://docs.microsoft.com/en-us/previous-versions/windows/internet-explorer/ie-developer/platform-apis/aa753582(v=vs.85)
//!
//! # Quick start
//!
//! In order to parse a *bookmark file*, you should use the [Netscape] struct.
//!
//! ```rust
//! use bookmarkt::Netscape;
//! use serde_json;
//! use std::path::Path;
//!
//! let path = Path::new("./res/netscape.html");
//! let parsed = Netscape::from_file(path).unwrap();
//!
//! assert_eq!(parsed.title, "Bookmarks");
//! assert_eq!(parsed.children.len(), 2);
//! ```
//!
//! The bookmarkt structures also support the *import* and *export* features.
//!
//! ```rust
//! use bookmarkt::Netscape;
//! use serde_json;
//! use std::path::Path;
//!
//! let path = Path::new("./res/firefox.html");
//! let imported = Netscape::from_file(path).unwrap();
//! let exported = imported.to_html().unwrap();
//! let reimported = Netscape::from_html(&exported).unwrap();
//!
//! assert_eq!(imported, reimported);
//! ```
//!
//! # Acknowledgment
//!
//! `bookmarkt` uses the following dependencies :
//! * [kuchiki](https://github.com/kuchiki-rs/kuchiki)
//! * [html5ever](https://github.com/servo/html5ever)
//!
//! I really appreciate these works and I hope you appreciate them too ;).
//!
//! I also took some ideas from these libraries :
//! * [Netscape-Bookmarks-File-Parser](https://github.com/FlyingWolFox/Netscape-Bookmarks-File-Parser)
//!
//! # Contributing
//!
//! Open a ticket on the [`bookmarkt` tracker](https://todo.sr.ht/~vlnk/bookmarkt).

#![deny(missing_docs)]

#[macro_use]
extern crate derive_builder;

mod collection;
mod node_ref_ext;

mod bookmark;
mod folder;
mod item;
mod netscape;

pub use bookmark::Bookmark;
pub use folder::Folder;
pub use item::Item;
pub use netscape::Netscape;
