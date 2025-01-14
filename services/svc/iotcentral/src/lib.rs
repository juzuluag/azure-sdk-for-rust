#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-1.1-preview")]
pub mod package_1_1_preview;
#[cfg(all(feature = "package-1.1-preview", not(feature = "no-default-version")))]
pub use package_1_1_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-04-30-preview")]
pub mod package_2021_04_30_preview;
#[cfg(all(feature = "package-2021-04-30-preview", not(feature = "no-default-version")))]
pub use package_2021_04_30_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-1.0")]
pub mod package_1_0;
#[cfg(all(feature = "package-1.0", not(feature = "no-default-version")))]
pub use package_1_0::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
