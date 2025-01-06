# Viper pip wrapper
Viper is a python project manager and pip wrapper based on Rust's cargo package manager. Viper is meant to make creating and managing python projects a centralized process.

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
"The Project" is the folder created by ```viper new```. By default, viper will create a virtual environment (venv) directory, a pyproject.toml file, a main.py file in the python entry point layout.
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
by passing the following at the end of the ```viper new``` command
- -e|--entry-point (entry point layout)
- -c|--class (class layout),
- -s|--simple (simple layout) 
### Managing Pip Packages
You may notice a ```[dependencies]``` section in the pyproject.toml file with a ```required```. This is an added section viper uses to know what packages you need to  install for you project. All you need to do is add the package name into the ```required``` list
```toml
[dependencies]
required = [
	"toml",
]
```
then run
```bash
viper reload
```
to reload your packages.
To remove packages, just delete the package name from the [dependencies] section of the pyproject.toml file
```toml
[dependencies]
required = []
```
and run
```bash
viper reload
```
to reload your packages again. Remember to run the viper reload command after changing project dependencies, otherwise those packages will not get added to the 
### Commands
Commands that manipulate your Project is as follows:
```bash
viper run    : if in a folder created by viper new, it will run the main.py file sourcing the local virtual environment
viper reload : if in a folder created by viper new, it will reload the packages listed under the [dependencies] section of the pyproject.toml file and install/remove packages as needed
```

## Future Plans
- [X] Overhauling the ```viper install``` and ```viper remove``` commands for a config.toml file in the root directory of your project similar in functionality to the Cargo.toml file in Rust.
	- note that the pyproject.toml file was chosen instead of config.toml
- [X] Moving source files to a src/ folder
- [ ] Adding the ability to search for pypi packages using viper. I can imagine how helpful this might be after pip removed their built in search feature
