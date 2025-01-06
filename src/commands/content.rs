pub const ENTRY_POINT: &str = 
r#"def main():
    print("hello, world!")


if __name__ == "__main__":
    main()
"#;

pub const CLASS: &str =
r#"class Main:
    def __init__(self):
        print("hello, world!")


if __name__ == "__main__":
    Main()
"#;

pub const SIMPLE: &str = 
r#"print("hello, world!")
"#;

pub const CONFIG_FILE: &str =
r#"[build-system]

[project]

[dependencies]
required = []"#;
