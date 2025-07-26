use std::fmt::Display;
use std::sync::Mutex;
use strum::EnumIter;
use wasm_bindgen::__rt::LazyLock;
use crate::models::status::Status;

static LAST_QUOTE: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));
#[derive(EnumIter, Debug, PartialEq, Eq, Clone)]
pub enum Topic {
    About,
    Contact,
    Cv,
    Donate,
    Quote,
    Social,
    Summary,
    Made,
}

const QUOTES: &[&str] = &[
    "\"Victory belongs to the most persevering.\" - Napoleon Bonaparte",
    "\"Sharing knowledge is the most fundamental act of friendship. Because it is a way you can give something without loosing something.\" - Richard Stallman",
    "\"The average consumer does not know the difference between browser, Internet and search box.\" - Mitchell Baker",
    "\"Never confuse a single defeat with a final defeat\" -  F. Scott Fitzgerald",
    "\"I mean, if 10 years from now, when you are doing something quick and dirty, you suddenly visualize that I am looking over your shoulders and say to yourself \"Dijkstra would not have liked this\", well, that would be enough immortality for me.\" - Edsger W. Dijkstra",
    "\"The question of whether a computer can think is no more interesting than the question of whether a submarine can swim.\" - Edsger W. Dijkstra",
    "\"The use of COBOL cripples the mind; its teaching should, therefore, be regarded as a criminal offense.\" - Edsger W. Dijkstra",
    "\"The most important property of a program is whether it accomplishes the intention of its user.\" - Graydon Hoare",
    "\"I think, fundamentally, open source does tend to be more stable software. It's the right way to do things.\" - Linus Torvalds",
    "\"Fully secure systems don't exist today and they won't exist in the future.\" - Adi Shamir",
    "\"Information is the resolution of uncertainty.\" - Claude Shannon",
    "\"Weak typing is a devil plaguing software correctness. It tempts you with ease of development, while secretly hiding undefined behaviour in the code.\" - Daniele Giachetto",
    "\"Only sneaky people and impostors can oppose the progress of sciences and can discredit them, because they are the only ones to whom the sciences do harm.\" - Friedrich der Große",
];

impl Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::About => "About",
                Self::Contact => "Contact",
                Self::Cv => "Cv",
                Self::Donate => "Donate",
                Self::Quote => "Quote",
                Self::Social => "Social",
                Self::Summary => "Summary",
                Self::Made => "Made with Rust! 🦀",
            }
        )
    }
}

impl Topic {
    pub fn get_link(&self) -> String {
        String::from(match self {
            Self::Donate => "https://paypal.me/danielegiachetto",
            Self::Social => "https://linkedin.com/in/danielegiachetto",
            Self::Contact => "mailto:work@danielegiachetto.com",
            Self::Cv => "https://github.com/RakuJa/CV/blob/master/CV.pdf",
            Self::Made => "https://github.com/orhun/ratzilla",
            _ => "",
        })
    }
    pub fn get_description(&self, status: Status) -> String {
        let tmp_val;
        String::from(match self {
            Self::About => {
                "
                     ███████████             █████                      ███
                    ░░███░░░░░███           ░░███                      ░░░
                     ░███    ░███   ██████   ░███ █████ █████ ████     █████  ██████
                     ░██████████   ░░░░░███  ░███░░███ ░░███ ░███     ░░███  ░░░░░███
                     ░███░░░░░███   ███████  ░██████░   ░███ ░███      ░███   ███████
                     ░███    ░███  ███░░███  ░███░░███  ░███ ░███      ░███  ███░░███
                     █████   █████░░████████ ████ █████ ░░████████     ░███ ░░████████
                    ░░░░░   ░░░░░  ░░░░░░░░ ░░░░ ░░░░░   ░░░░░░░░      ░███  ░░░░░░░░
                                                                   ███ ░███
                                                                  ░░██████
                                                                   ░░░░░░

                    I'm Daniele Giachetto aka RakuJa, a Software developer & Cybersecurity major.
                    This is an interactive website, in which you'll use the TUI with your keyboard to know more about myself."

            }
            Self::Cv => {
                "https://github.com/RakuJa/CV/blob/master/CV.pdf"
            },
            Self::Contact => {
                "mailto:work@danielegiachetto.com \n\
                mailto:education@danielegiachetto.com \n\
                mailto:personal@danielegiachetto.com
                "
            }
            Self::Donate => {
                "https://paypal.me/danielegiachetto \n\
                https://ko-fi.com/rakuja
                "
            },
            Self::Quote => {
                tmp_val = match status {
                    Status::Completed => LAST_QUOTE.lock().map_or(String::new(), |lq| lq.clone()),
                    Status::Todo => {
                        let quote = get_random_quote();
                        if let Ok(mut lq) = LAST_QUOTE.lock() {
                            (*lq).clone_from(&quote);
                        }
                        quote
                    }
                };
                tmp_val.as_str()
            },
            Self::Social => {
                "https://linkedin.com/in/danielegiachetto \n\
                https://github.com/rakuja
                "

            },
            Self::Summary => {
                "
                                                     ./o.                  🚗 My daily drivers: EndeavourOS | CachyOS
                                                   ./sssso-                --------------------
                                                  :osssssss+-              📡 ABOUT
                                                :+sssssssssso/.            🌌 whoami => Daniele Giachetto
                                              -/ossssssssssssso/.          📑 Resume 🔽
                                            -/+sssssssssssssssso+:         🔗 https://github.com/RakuJa/CV/blob/master/CV.pdf
                                          -:/+sssssssssssssssssso+/.       -----------
                                        .://osssssssssssssssssssso++-      🎉 SOCIALS 🔽
                                      .://+ssssssssssssssssssssssso++:     💻 https://github.com/rakuja
                                    .:///ossssssssssssssssssssssssso++:    🏢 https://linkedin.com/in/danielegiachetto
                                   :////ssssssssssssssssssssssssssso+++.   -----------
                                 -////+ssssssssssssssssssssssssssso++++-   🎁 DONATE 🔽
                                  ..-+oosssssssssssssssssssssssso+++++/    💰 https://paypal.me/danielegiachetto
                                   ./++++++++++++++++++++++++++++++/:.     💸 https://ko-fi.com/rakuja
                                   :::::::::::::::::::::::::------

 mailto:work@danielegiachetto.com | mailto:education@danielegiachetto.com | mailto:personal@danielegiachetto.com
                "
            },
            Self::Made => "I can't hold my inner voice. He tells me to rewrite the complete universe with Rust \n\
            MADE WITH ♥ using the fantastic Ratzilla library => https://github.com/orhun/ratzilla",
        })
    }
}

fn get_random_quote() -> String {
    (*fastrand::choice(QUOTES).unwrap_or(
        &"\"Victory belongs to the most persevering.\" - Napoleon Bonaparte"
    )).to_string()
}
