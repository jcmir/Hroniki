#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feature {
    AI,
    CloudSync,
    FamilyArchive,
    UnlimitedObjects,
    ExportBackup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscriptionPlan {
    Free,
    Pro,
    Family,
    Enterprise,
}

impl SubscriptionPlan {
    pub fn features(&self) -> Vec<Feature> {
        match self {
            SubscriptionPlan::Free => vec![
                Feature::ExportBackup,
            ],
            SubscriptionPlan::Pro => vec![
                Feature::ExportBackup,
                Feature::UnlimitedObjects,
                Feature::AI,
                Feature::CloudSync,
            ],
            SubscriptionPlan::Family => vec![
                Feature::ExportBackup,
                Feature::UnlimitedObjects,
                Feature::AI,
                Feature::CloudSync,
                Feature::FamilyArchive,
            ],
            SubscriptionPlan::Enterprise => vec![
                Feature::ExportBackup,
                Feature::UnlimitedObjects,
                Feature::AI,
                Feature::CloudSync,
                Feature::FamilyArchive,
            ],
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pro" => SubscriptionPlan::Pro,
            "family" => SubscriptionPlan::Family,
            "enterprise" => SubscriptionPlan::Enterprise,
            _ => SubscriptionPlan::Free,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            SubscriptionPlan::Free => "free",
            SubscriptionPlan::Pro => "pro",
            SubscriptionPlan::Family => "family",
            SubscriptionPlan::Enterprise => "enterprise",
        }
    }
}
