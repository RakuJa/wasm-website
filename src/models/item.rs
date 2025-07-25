#[derive(Debug)]
pub struct Item {
    pub title: String,
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

const TITLES: &[&str] = &[
    "about",
    "contacts",
    "cv",
    "donate",
    "quote",
    "socials",
    "summary",
    "Made with Rust! 🦀",
];

impl Item {
    pub fn get_list_of_titles() -> Vec<String> {
        TITLES
            .iter()
            .map(std::string::ToString::to_string)
            .collect()
    }
    pub fn get_link(&self) -> String {
        String::from(match self.title.to_uppercase().as_str() {
            "DONATE" => "https://paypal.me/danielegiachetto",
            "SOCIALS" => "https://linkedin.com/in/danielegiachetto",
            "CONTACTS" => "mailto:work@danielegiachetto.com",
            "CV" => "https://github.com/RakuJa/CV/blob/master/CV.pdf",
            "MADE WITH RUST! 🦀" => "https://github.com/orhun/ratzilla",
            _ => "",
        })
    }
    pub fn get_description(&self) -> String {
        String::from(match self.title.to_uppercase().as_str() {
            "ABOUT" => {
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
            "CV" => {
                "https://github.com/RakuJa/CV/blob/master/CV.pdf"
            },
            "CONTACTS" => {
                "mailto:work@danielegiachetto.com \n\
                mailto:education@danielegiachetto.com \n\
                mailto:personal@danielegiachetto.com
                "
            }
            "DONATE" => {
                "https://paypal.me/danielegiachetto \n\
                https://ko-fi.com/rakuja
                "
            },
            "QUOTE" => {
                fastrand::choice(QUOTES).unwrap_or(
                    &"\"Victory belongs to the most persevering.\" - Napoleon Bonaparte"
                )
            },
            "SOCIALS" => {
                "https://linkedin.com/in/danielegiachetto \n\
                https://github.com/rakuja
                "

            },
            "SUMMARY" => {
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
            "MADE WITH RUST! 🦀" => "I can't hold my inner voice. He tells me to rewrite the complete universe with Rust \n\
            MADE WITH ♥ using the fantastic Ratzilla library => https://github.com/orhun/ratzilla",
            _ => "",
        })
    }
}
