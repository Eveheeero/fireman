/// Third-party dependency license metadata shared across CLI and TUI.
pub(crate) struct ThirdPartyDep {
    pub(crate) name: &'static str,
    pub(crate) version: &'static str,
    pub(crate) license: &'static str,
    pub(crate) url: &'static str,
}

pub(crate) const PROJECT_LICENSE: &str = "GPL-2.0-only";
pub(crate) const PROJECT_URL: &str = "https://github.com/Eveheeero/fireman";
pub(crate) const PROJECT_COPYRIGHT: &str = "Copyright (C) 2024 Eveheeero <xhve00000@gmail.com>";

pub(crate) const THIRD_PARTY_DEPS: &[ThirdPartyDep] = &[
    ThirdPartyDep {
        name: "capstone-rs",
        version: "0.14.0",
        license: "MIT",
        url: "https://github.com/capstone-rust/capstone-rs",
    },
    ThirdPartyDep {
        name: "Capstone Engine",
        version: "",
        license: "BSD-3-Clause",
        url: "https://github.com/capstone-engine/capstone",
    },
    ThirdPartyDep {
        name: "unicorn-engine",
        version: "2.1.5",
        license: "GPL-2.0",
        url: "https://github.com/unicorn-engine/unicorn",
    },
    #[cfg(feature = "keystone")]
    ThirdPartyDep {
        name: "keystone-engine",
        version: "0.1.0",
        license: "GPL-2.0",
        url: "https://github.com/keystone-engine/keystone",
    },
];

/// Format the full license text block used by `--license` and the TUI overlay.
pub(crate) fn format_license_text(program_name: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{program_name} — {PROJECT_COPYRIGHT}\n\
         Licensed under the GNU General Public License v2.0 ({PROJECT_LICENSE}).\n\
         Source: {PROJECT_URL}\n\
         \n\
         This program is distributed in the hope that it will be useful,\n\
         but WITHOUT ANY WARRANTY; without even the implied warranty of\n\
         MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n\
         GNU General Public License for more details.\n\
         \n\
         Third-party libraries:\n"
    ));
    for dep in THIRD_PARTY_DEPS {
        if dep.version.is_empty() {
            out.push_str(&format!(
                "  {:<25} — {} ({})\n",
                dep.name, dep.license, dep.url
            ));
        } else {
            out.push_str(&format!(
                "  {:<25} — {} ({})\n",
                format!("{} {}", dep.name, dep.version),
                dep.license,
                dep.url
            ));
        }
    }
    out.push_str("\nSee THIRD_PARTY_LICENSES for full license texts.\n");
    out
}
