use std::io::Write;

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f64,
    pub samples: u32,
    pub depth: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 512,
            height: 288,
            aspect_ratio: 16.0 / 9.0,
            samples: 10,
            depth: 5,
        }
    }
}

impl Config {
    pub fn load(path: &std::path::Path) -> Self {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error when loading config : {},\n fallback to default", e);
                String::from("")
            }
        };

        let mut config = Self::default();
        let lines = content.split("\n");
        let mut line_count = 1;

        for line in lines {
            // empty line 
            if line == "" {
                continue;
            }

            // lex current line
            let line_content: Vec<&str> = line.split("=").map(|i| i.trim()).collect();

            // check if line is correct
            if line_content.len() != 2 {
                eprintln!(
                    "in {2:?}, error at line {0}: \n\texpected \"ident = value\", got \"{1}\" \n\tfallback to default", 
                    line_count, 
                    line_content[0],
                    path
                );
                return Self::default();
            }

            match line_content[0] {
                "width" => config.width = line_content[1].parse::<u32>().unwrap(),
                "height" => config.height = line_content[1].parse::<u32>().unwrap(),
                "samples" => config.samples = line_content[1].parse::<u32>().unwrap(),
                "depth" => config.depth = line_content[1].parse::<u32>().unwrap(),
                _ => {
                    eprintln!(
                        "in {2:?}, error at line {0}: \n\tunknown pattern: {1}",
                        line_count, line_content[0], path
                    );
                    return Self::default();
                }
            }

            line_count += 1;
        }

        config.aspect_ratio = config.width as f64 / config.height as f64;
        config
    }

    pub fn save(&self, path: &std::path::Path) {
        let mut file = std::fs::File::create(path).unwrap();
        file.write(format!("width = {}\n", self.width).as_bytes()).unwrap();
        file.write(format!("height = {}\n", self.height).as_bytes()).unwrap();
        file.write(format!("samples = {}\n", self.samples).as_bytes()).unwrap();
        file.write(format!("depth = {}\n", self.depth).as_bytes()).unwrap();
    }
}
