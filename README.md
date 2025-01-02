# Viper pip wrapper
Viper is a python project manager and pip wrapper written in Rust and based on Rust's cargo package manager. Viper is meant to make creating and managing python projects a centralized process taking inspiration from cargo.

## The Workshop
### Description
"The Worksop" is an alias Viper uses to refer to the folder your python projects are held. This (at the moment) is really only used in the context of the source code.
### Commands
Commands that manipulate your Workshop are as follows:
```bash
viper new : creates a new python project with a virtual environment and a main.py file. You can pass in a few arguments to get different main.py layouts
```

## The Project
### Description
"The Project" is the folder created by ```viper new```. By default, viper will create a virtual environment (venv) directory and a main.py file in the python entry point layout.
```python
def main():
    print("hello, world!")


if __name__ == "__main__":
    main()
```
You also have the ability to format the main.py file in two other ways referred to as the class layout
```python
class Main:
    def __init__(self):
        print("hello, world!")


if __name__ == "__main__":
    Main()
```
and the simple layout
```python
print("hello, world!")
```
by passing either the -e|--entry-point (entry point layout), -c|--class (class layout), or -s|--simple (simple layout) at the end of the ```viper new``` command
### Commands
Commands that manipulate your Project is as follows:
```bash
viper run     : if in a folder created by viper new, it will run the main.py file sourcing the local virtual environment
viper list    : if in a folder created by viper new, it will return the list of installed packages in the local virtual environment
viper install : if in a folder created by viper new, it will install the specified package to the local virtual environment
viper remove  : if in a folder created by viper new, it will remove the specified package from the local virtual environment
```

## Future Plans
- Overhauling the ```viper install``` and ```viper remove``` commands for a config.toml file in the root directory of your project similar in functionality to the Cargo.toml file in Rust.
