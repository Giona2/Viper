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


//pub fn default_content() -> String {
//	format!("def main():\n{}print({})\n\n\nif __name__ == {}:\n	main()\n", r#""hello, world!""#, r#""__main__""#)
//}
//
//pub fn class_content() -> String {
//	format!("class Main:\n	def __init__(self):\n		print({})\n\n\nif __name__ == {}:\n	Main()\n", r#""hello, world!""#, r#""__main__""#)
//}
//
//pub fn simple_content() -> String {
//	r#"print("hello, world!")"#.to_string()
//}

pub const CONFIG_FILE: &str =
r#"[build-system]

[project]

[dependencies]
required = []"#;
