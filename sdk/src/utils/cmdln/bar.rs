use std::{cmp::min, fmt::Display};

#[cfg(feature = "cmdln")]
pub struct Bar {
    bracket_left: String,
    bracket_right: String,
    fill: String,
    blank: String,
    width: usize,
    progress: usize,
    progress_label: Option<String>,
    label_left: Option<String>,
    label_right: Option<String>,
}

#[cfg(feature = "cmdln")]
impl Bar {
    pub fn new(
        bracket_left: String,
        bracket_right: String,
        fill: char,
        blank: char,
        width: usize,
    ) -> Self {
        Bar {
            bracket_left,
            bracket_right,
            fill: String::from(fill),
            blank: String::from(blank),
            width,
            progress: 0,
            progress_label: None,
            label_left: None,
            label_right: None,
        }
    }

    pub fn progress(&mut self, progress: usize, label: Option<String>) {
        assert!(
            progress <= self.width,
            "Progress percentage must not be bigger than the width"
        );
        if let Some(label) = &label {
            assert!(
                label.len() <= self.width,
                "Progress label must not be bigger than the bar width"
            );
        }

        self.progress = progress;
        self.progress_label = label;
    }

    pub fn labels(&mut self, label_left: Option<String>, label_right: Option<String>) {
        let mut total_label_len = 0;
        if let Some(label) = &label_left {
            total_label_len += label.len();
        }
        if let Some(label) = &label_right {
            total_label_len += label.len();
        }
        assert!(
            total_label_len < self.bracket_left.len() + self.width + self.bracket_right.len(),
            "Labels must combined be less than the total bar width"
        );

        self.label_left = label_left;
        self.label_right = label_right;
    }
}

#[cfg(feature = "cmdln")]
impl Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        // labels
        let mut label_offset = 0;
        if let Some(label) = &self.label_left {
            out.push_str(label);
            label_offset += label.len();
        }
        if let Some(label) = &self.label_right {
            out.push_str(&format!(
                "{}{}\n",
                " ".repeat(
                    self.bracket_left.len() + self.width + self.bracket_right.len()
                        - label_offset
                        - label.len()
                ),
                label
            ));
        }

        // bar
        out.push_str(&format!(
            "{}{}{}{}\n",
            self.bracket_left,
            self.fill.repeat(self.progress),
            self.blank.repeat(self.width - self.progress),
            self.bracket_right
        ));

        // progress label
        if let Some(label) = &self.progress_label {
            out.push_str(&format!(
                "{}^\n{}{}",
                " ".repeat(min(self.progress + self.bracket_left.len(), self.width)),
                " ".repeat(min(
                    self.progress + self.bracket_left.len() + 1,
                    self.width + self.bracket_left.len() - label.len()
                )),
                label
            ));
        }

        write!(f, "{out}")
    }
}
