# Viper Pip Wrapper
Viper is a python project manager and pip wrapper based on Rust's cargo package manager. Viper is meant to make creating and managing python projects a centralized process.

## How to use
Viper presents a steamlined method of managing python projects on the user's computer, so naturally viper will be exceedingly easy to use (at least in my opinion)
### Create a new project
First, navigate to your projects folder (or wherever you want to create a new project)
```bash
cd ~/path/to/projects
```
To create a new project, just run
```bash
viper new <project name>
```
and replace the ```<project name>``` field with the name of your project. Be sure to use common characters like lowercase letters, underscores, and numbers in the project name to minimize possible problems in the future (ex, first_project).
```viper new``` will create a directory with the same name as the project name, then a src directory with a main.py entry file, a virtual environment (venv) folder, and a pyproject.toml file
```bash
<project name>
|- venv
|- src
   |- main.py
|- pyproject.toml
```
### main.py
By default, main.py will be formatted in the entry point format
```python
def main():
    print("hello, world!")


if __name__ == "__main__":
    main()
```
The entry point format is well known for it verbosity, so it was chosen for the default. There are two formats you can use, referred to as the class and simple format
```python
# class layout
class Main:
    def __init__(self):
        print("hello, world!")


if __name__ == "__main__":
    Main()

# simple layout
print("hello, world!")
```
To tell viper to use either of these formats, you need to pass in the format tag after your project name in ```viper run```. Assuming the name of the project is first_project, the arguments you can pass in are as follows:
```bash
viper first_project -e | viper first_project --entry-point
viper first_project -c | viper first_project --class
viper first_project -s | viper first_project --simple
```
See below to find out how to install packages to your project
### pyproject.toml
For reference, this is the default layout of the pyproject.toml file
```toml
[build-system]
requires = ["pip", "venv", "viper"]

[dependencies]
required = []

[project]
name = "<project name>"
version = "1.0.0"
```
The pyproject.py file details the project's metadata, mainly the required programs it needs to run ```[build system]```, the python packages it needs to install ```[dependencies]```, and the specific metadata like the project name and version ```[project]```  
As some python enthusiasts may notice, this file is meant to compatibilize with pypi, the only added difference being the ```[dependencies]``` field. To my knowlege, this shouldn't interfere if you wish to upload your project to pypi, but feel free to post a complaint about it if it does

## Commands
The following is the output of the ```viper help``` command
```bash
--<blue>   = tags
  <yellow> = arguments

viper new <project name> --<main.py layout>
  creates a new python project with a virtual environment and a main.py file. You can pass in a few arguments to get different main.py layouts
    -e | --entry-point: Creates the main.py in the entry point layout
    -c | --class      : Creates the main.py in the object-oriented layout
    -s | --simple     : Creates the main.py in the most simplistic layout as possible

viper run -- <arguments>
  if in a folder created by viper new, it will run the main.py file sourcing the local virtual environment
  any argument you pass in after the "--" will be forwarded to the main.py file

viper reload
  if in a folder created by viper new, it will reload the packages listed under the [dependencies] section of the pyproject.toml file and install/remove packages as needed

viper search <package name>
  searches pydigger.com for the package name and version number of all matched packages
```

## Future Plans
- [X] Overhauling the ```viper install``` and ```viper remove``` commands for a config.toml file in the root directory of your project similar in functionality to the Cargo.toml file in Rust.
	- note that the pyproject.toml file was chosen instead of config.toml
- [X] Moving source files to a src/ folder
- [X] Adding the ability to search for pypi packages using viper. I can imagine how helpful this might be after pip removed their built in search feature
	- Note that viper searches [pydigger](https://pydigger.com/) rather than pypi for packages for better compadibility
- [ ] Implement better error handling and edge-cases rather than relying on Rust's built in error handler
