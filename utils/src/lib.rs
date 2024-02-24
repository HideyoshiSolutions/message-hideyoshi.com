pub enum ResourceType {
    OPEN,
    USER,
    ADMIN,
}

impl ResourceType {
    fn as_str(&self) -> &str {
        match self {
            ResourceType::OPEN => "OPEN",
            ResourceType::USER => "USER",
            ResourceType::ADMIN => "ADMIN",
        }
    }

    fn from_str(s: &str) -> ResourceType {
        match s {
            "OPEN" => ResourceType::OPEN,
            "USER" => ResourceType::USER,
            "ADMIN" => ResourceType::ADMIN,
            _ => panic!("Invalid resource type"),
        }
    }
}
