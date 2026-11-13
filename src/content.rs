//! Everything the page says lives here. Add or change entries and the
//! components pick them up

pub struct Profile {
    pub name: &'static str,
    pub location: &'static str,
    pub characteristics: &'static str,
    pub tagline: &'static str,
    pub greeting: &'static str,
    pub readme_title: &'static str,
    pub github: &'static str,
}

pub struct Education {
    pub school: &'static str,
    pub program: &'static str,
    /// Lines shown under the program, each prefixed with an arrow
    pub details: &'static [&'static str],
    pub years: &'static str,
}

pub struct Project {
    pub name: &'static str,
    /// Short label shown on the right, e.g. "Hackathon". Empty for none
    pub tag: &'static str,
    /// Lines shown under the name, each prefixed with an arrow
    pub details: &'static [&'static str],
}

pub struct Tech {
    pub name: &'static str,
    /// Colour of the square shown before the name. Mostly brand colours,
    /// lightened where the real one disappears on the dark panel
    pub color: &'static str,
}

pub struct ContactLink {
    pub label: &'static str,
    pub href: &'static str,
    pub icon: &'static str,
    /// Spans the full width of the contact card
    pub wide: bool,
}

pub const PROFILE: Profile = Profile {
    name: "Hadi Osmani",
    location: "Ottawa, Ontario, Canada",
    characteristics: "he/him · 18 years old",
    tagline: "Software Engineering Intern at Daybook Health",
    greeting: "👋 I'm Hadi, a tech enthusiast from Canada 🇨🇦",
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
        program: "Highschool Diploma",
        details: &[
            "2× Honour Roll (throughout 2022-2026)",
            "Ontario Scholar",
            "Bronze Community Service Award",
            "Technology Student Certificate",
        ],
        years: "2022 – 2026",
    },
];

pub const PROJECTS: &[Project] = &[
    Project {
        name: "Guardia",
        tag: "Hackathon",
        details: &[
            "Python (LIEF & Capstone)",
            "Flags possible malware using static analysis",
        ],
    },
    Project {
        name: "Astraeus",
        tag: "Contributed",
        details: &[
            "Rust + zstd",
            "Built a compressed file system for persistent save/load",
        ],
    },
    Project {
        name: "Botcore",
        tag: "",
        details: &[
            "Python (Discord, Cogs, Logging)",
            "A ready-made base for building custom Discord bots",
        ],
    },
];

pub const STACK: &[Tech] = &[
    Tech { name: "Rust", color: "#e8b48f" },
    Tech { name: "Python", color: "#5b9bd5" },
    Tech { name: "WebAssembly", color: "#654ff0" },
    Tech { name: "Java", color: "#ed8b00" },
    Tech { name: "Git", color: "#f05032" },
    Tech { name: "Bash", color: "#4eaa25" },
    Tech { name: "Codium", color: "#2f80ed" },
    Tech { name: "Neovim", color: "#57a143" },
    Tech { name: "Claude", color: "#d97757" },
];

pub const CONTACT: &[ContactLink] = &[
    ContactLink { label: "LinkedIn", href: "https://www.linkedin.com/in/hadi-osmani/", icon: "linkedin", wide: false },
    ContactLink { label: "Email", href: "mailto:contact@hadi.rocks", icon: "mail", wide: false },
    ContactLink { label: "Signal", href: "https://signal.me/#eu/Li5oLbM3sx28t_UTMs31C4euBL6At56NU4YwcPdLbLMuryuXdGpJTqebgxHRpE89", icon: "signal", wide: true },
    ContactLink { label: "Book a meeting", href: "https://cal.com/hadi-osmani", icon: "calendar", wide: true },
];

