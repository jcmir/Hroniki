#[derive(Debug, Clone)]
pub enum DomainEvent {
    // Identity & Session Events
    UserRegistered {
        user_id: String,
        email: Option<String>,
    },
    UserAuthenticated {
        user_id: String,
        success: bool,
    },
    SessionOpened {
        session_id: String,
        user_id: String,
        device_name: Option<String>,
    },
    SessionClosed {
        session_id: String,
    },
    AuthenticationRequired,
    SessionRestored,

    // Chronology Events
    CategoryCreated {
        category_id: String,
        name: String,
    },
    ObjectCreated {
        object_id: String,
        name: String,
        category_id: String,
    },
    EntryCreated {
        entry_id: String,
        object_id: String,
    },

    // Backup & Restore Events
    ArchiveExported {
        user_id: Option<String>,
        path: String,
    },
    ArchiveImported {
        user_id: Option<String>,
        success: bool,
    },

    // Licensing & Subscription Events
    PlanUpdated {
        user_id: String,
        plan: String,
        updated_at: String,
    },

    // Application Lifecycle Events
    ApplicationStarted,
    ApplicationSuspended,
    ApplicationResumed,
    ApplicationClosed,
    ApplicationLocked,
}
