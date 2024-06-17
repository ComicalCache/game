use std::{
    cmp::{max, min},
    fmt::Display,
};

#[cfg(feature = "cmdln")]
pub struct RatioBar {
    bracket_left: String,
    bracket_right: String,
    fill: String,
    blank: String,
    separator: String,
    width: usize,
    progress_left: usize,
    progress_label_left: Option<String>,
    progress_right: usize,
    progress_label_right: Option<String>,
    label_left: Option<String>,
    label_right: Option<String>,
}

#[cfg(feature = "cmdln")]
impl RatioBar {
    pub fn new(
        bracket_left: String,
        bracket_right: String,
        fill: char,
        blank: char,
        separator: char,
        width: usize,
    ) -> Self {
        assert!(width % 2 == 1, "Width must be uneven");

        RatioBar {
            bracket_left,
            bracket_right,
            fill: String::from(fill),
            blank: String::from(blank),
            separator: String::from(separator),
            width,
            progress_left: 0,
            progress_label_left: None,
            progress_right: 0,
            progress_label_right: None,
            label_left: None,
            label_right: None,
        }
    }

    pub fn progress(
        &mut self,
        progress_left: usize,
        label_left: Option<String>,
        progress_right: usize,
        label_right: Option<String>,
    ) {
        assert!(
            progress_left <= self.width / 2,
            "Progress left must not be bigger than the width"
        );
        assert!(
            progress_right <= self.width / 2,
            "Progress right must not be bigger than the width"
        );

        if let Some(label) = &label_left {
            assert!(
                label.len() <= self.width / 2,
                "Progress label left must not be bigger than the bar width"
            );
        }
        if let Some(label) = &label_right {
            assert!(
                label.len() <= self.width / 2,
                "Progress label right must not be bigger than the bar width"
            );
        }

        self.progress_left = progress_left;
        self.progress_right = progress_right;
        self.progress_label_left = label_left;
        self.progress_label_right = label_right;
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
impl Display for RatioBar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write; // for writing to string

        let half_width = self.width / 2;
        let mut out = String::new();

        // labels
        let mut label_offset = 0;
        if let Some(label) = &self.label_left {
            write!(out, "{label}")?;
            label_offset += label.len();
        }
        if let Some(label) = &self.label_right {
            write!(
                out,
                "{}{}\n",
                " ".repeat(
                    self.bracket_left.len() + self.width + self.bracket_right.len()
                        - label_offset
                        - label.len()
                ),
                label
            )?;
        }

        // bar
        write!(
            out,
            "{}{}{}{}{}{}{}\n",
            self.bracket_left,
            self.blank.repeat(half_width - self.progress_left),
            self.fill.repeat(self.progress_left),
            self.separator,
            self.fill.repeat(self.progress_right),
            self.blank.repeat(half_width - self.progress_right),
            self.bracket_right
        )?;

        // progress labels
        // has to be assembled in a two stage process, first figure out (if needed) space between ^
        // then figure out (if needed) space between labels + handle overflows
        let mut buffer = String::new();
        if let Some(_) = &self.progress_label_left {
            write!(
                buffer,
                "{}^{}",
                " ".repeat(max(
                    // < 100% progress
                    self.bracket_left.len() + half_width - self.progress_left - 1,
                    // = 100% progress, else would overflow bar width to the left
                    self.bracket_left.len()
                )),
                " ".repeat(
                    min(
                        // < 100% progress
                        self.progress_left,
                        // = 100% progress, else would overflow bar width to the left
                        half_width - 1
                    ) + self.blank.len()
                )
            )?;
        } else {
            write!(
                buffer,
                "{}",
                " ".repeat(self.bracket_left.len() + half_width + 1)
            )?;
        }
        if let Some(_) = &self.progress_label_right {
            write!(
                buffer,
                "{}^\n",
                " ".repeat(min(
                    // < 100% progress
                    self.progress_right,
                    // = 100% progress, else would overflow bar width to the right
                    half_width - 1
                ))
            )?;
        } else {
            write!(buffer, "\n")?;
        }
        write!(out, "{buffer}")?;
        buffer.clear();

        if let Some(label) = &self.progress_label_left {
            // needed because special case where progress is 100%, and label is moved right by one
            let offset = if half_width - self.progress_left == 0 {
                1
            } else {
                0
            };

            write!(
                buffer,
                "{}{label}{}",
                " ".repeat(min(
                    // label wouldn't overflow bar width
                    max(
                        // < 100% progress
                        half_width - self.progress_left,
                        // = 100% progress, else would overflow bar width to the left
                        1
                    ) + self.bracket_left.len(),
                    // label would overflow bar width to the right
                    half_width + self.bracket_left.len() - label.len()
                )),
                " ".repeat(
                    max(
                        // label wouldn't overflow bar width
                        self.progress_left,
                        // label would overflow bar width to the right
                        label.len()
                    ) + self.blank.len()
                        - label.len()
                        - offset
                )
            )?;
        } else {
            write!(
                buffer,
                "{}",
                " ".repeat(self.bracket_left.len() + half_width + 1)
            )?;
        }
        if let Some(label) = &self.progress_label_right {
            write!(
                buffer,
                "{}{label}",
                " ".repeat(min(
                    // label wouldn't overflow bar width
                    self.progress_right + 1,
                    // label would overflow bar width to the right
                    half_width - label.len()
                )),
            )?;
        }
        write!(out, "{buffer}")?;

        write!(f, "{out}")
    }
}
