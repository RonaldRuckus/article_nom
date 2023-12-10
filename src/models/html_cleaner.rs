use regex::Regex;

#[derive(Clone)]
pub struct CleanerConfig {
    pub remove_script_tags: bool,
    pub remove_a_tags: bool,
    pub remove_img_tags: bool,
    pub remove_source_tags: bool
}

pub struct HtmlCleaner {
    remove_script_tags: bool,
    remove_a_tags: bool,
    remove_img_tags: bool,
    remove_source_tags: bool
}

impl HtmlCleaner {
    pub fn new() -> HtmlCleaner {
        HtmlCleaner {
            remove_script_tags: false,
            remove_a_tags: false,
            remove_img_tags: false,
            remove_source_tags: false
        }
    }

    pub fn apply_config(mut self, config: &CleanerConfig) -> Self {
        self.remove_script_tags = config.remove_script_tags;
        self.remove_a_tags = config.remove_a_tags;
        self.remove_img_tags = config.remove_img_tags;
        self.remove_source_tags = config.remove_source_tags;

        self
    }

    pub fn clean(&self, input: &str) -> String {
        let mut clean_text = input.to_string();

        if self.remove_script_tags {
            let script_tag_pattern = r"(?is)<script.*?</script>";
            clean_text = Regex::new(script_tag_pattern)
                .unwrap()
                .replace_all(&clean_text, "")
                .to_string();
        }

        if self.remove_a_tags {
            let a_tag_pattern = r"(?i)<a\s+[^>]*>|</a>";
            clean_text = Regex::new(a_tag_pattern)
                .unwrap()
                .replace_all(&clean_text, "")
                .to_string();
        }

        if self.remove_img_tags {
            let img_tag_pattern = r"(?i)<img\s+[^>]*>";
            clean_text = Regex::new(img_tag_pattern)
                .unwrap()
                .replace_all(&clean_text, "")
                .to_string();
        }

        if self.remove_source_tags {
            let source_tag_pattern = r"(?i)<source\s+[^>]*>";
            clean_text = Regex::new(source_tag_pattern)
                .unwrap()
                .replace_all(&clean_text, "")
                .to_string();
        }

        clean_text
    }
}
