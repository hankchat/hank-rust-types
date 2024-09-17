// @generated
#[cfg(feature = "hank")]
// @@protoc_insertion_point(attribute:hank)
pub mod hank {
    include!("hank.rs");
    // @@protoc_insertion_point(hank)
    #[cfg(feature = "hank-cron")]
    // @@protoc_insertion_point(attribute:hank.cron)
    pub mod cron {
        include!("hank.cron.rs");
        // @@protoc_insertion_point(hank.cron)
    }
    #[cfg(feature = "hank-database")]
    // @@protoc_insertion_point(attribute:hank.database)
    pub mod database {
        include!("hank.database.rs");
        // @@protoc_insertion_point(hank.database)
    }
    #[cfg(feature = "hank-message")]
    // @@protoc_insertion_point(attribute:hank.message)
    pub mod message {
        include!("hank.message.rs");
        // @@protoc_insertion_point(hank.message)
    }
    #[cfg(feature = "hank-plugin")]
    // @@protoc_insertion_point(attribute:hank.plugin)
    pub mod plugin {
        include!("hank.plugin.rs");
        // @@protoc_insertion_point(hank.plugin)
    }
}

pub use hank::*;
