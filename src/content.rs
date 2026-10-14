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
    /// Lines shown under the program, each prefixed with an arrow.
    pub details: &'static [&'static str],
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
    /// Spans the full width of the contact card.
    pub wide: bool,
}

pub const PROFILE: Profile = Profile {
    name: "Hadi Osmani",
    location: "Ottawa, Ontario, Canada",
    pronouns: "he/him",
    tagline: "Tech enthusiast & Software Engineering Intern at Daybook",
    greeting: "👋 I'm Hadi, a tech enthusiast from Canada.",
    readme_title: "hadirocks/README.md",
    github: "https://github.com/x7lex",
};

pub const EDUCATION: &[Education] = &[
    Education {
        school: "Carleton University",
        program: "Computer Science (Honours)",
        details: &["School of Computer Science"],
        years: "Year 1",
    },
    Education {
        school: "St. Francis Xavier High School",
        program: "Highschool Degree",
        details: &[
            "2× Honour Roll (throughout 2022-2026)",
            "Ontario Scholar",
            "Bronze Community Service Award",
            "Technology Student Certificate",
        ],
        years: "2022 – 2026",
    },
];

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
    ContactLink { label: "LinkedIn", href: "https://www.linkedin.com/in/hadi-osmani/", icon: "linkedin", wide: false },
    ContactLink { label: "Email", href: "mailto:contact@hadi.rocks", icon: "mail", wide: false },
    ContactLink { label: "Signal", href: "#", icon: "signal", wide: true },
    ContactLink { label: "Book a meeting", href: "#", icon: "calendar", wide: true },
];

