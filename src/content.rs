//! Everything the page says lives here. Add or change entries and the
//! components pick them up.

pub struct Profile {
    pub name: &'static str,
    pub location: &'static str,
    pub pronouns: &'static str,
    pub tagline: &'static str,
    pub greeting: &'static str,
    pub readme_title: &'static str,
    pub github: &'static str,
}

pub struct Education {
    pub school: &'static str,
    pub program: &'static str,
    pub faculty: &'static str,
    pub years: &'static str,
}

pub struct Tech {
    pub name: &'static str,
    /// Base name of the icon in `assets/icons/`.
    pub icon: &'static str,
}

pub struct ContactLink {
    pub label: &'static str,
    pub href: &'static str,
    pub icon: &'static str,
}

pub const PROFILE: Profile = Profile {
    name: "Hadi Osmani",
    location: "Ottawa, Ontario, Canada",
    pronouns: "he/him",
    tagline: "tech hobbyist, software intern @ daybook",
    greeting: "👋 I'm Hadi, a tech hobbyist from Canada.",
    readme_title: "hadirocks/README.md",
    github: "https://github.com/x7lex",
};

pub const EDUCATION: &[Education] = &[Education {
    school: "Carleton University",
    program: "Computer Science (Honours)",
    faculty: "School of Computer Science",
    years: "2026 – present",
}];

pub const STACK: &[Tech] = &[
    Tech { name: "Python", icon: "python" },
    Tech { name: "Rust", icon: "rust" },
    Tech { name: "WebAssembly", icon: "webassembly" },
    Tech { name: "Linux", icon: "linux" },
    Tech { name: "Git", icon: "git" },
    Tech { name: "VS Code", icon: "vscode" },
    Tech { name: "Claude", icon: "claude" },
];

// TODO: replace the "#" placeholders with real links.
pub const CONTACT: &[ContactLink] = &[
    ContactLink { label: "LinkedIn", href: "https://www.linkedin.com/in/hadi-osmani/", icon: "linkedin" },
    ContactLink { label: "Email", href: "mailto:contact@hadi.rocks", icon: "mail" },
    ContactLink { label: "Signal", href: "#", icon: "signal" },
    ContactLink { label: "Book a meeting", href: "#", icon: "calendar" },
];
