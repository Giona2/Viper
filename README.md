# Viper Pip Wrapper
Viper is a python project manager and pip wrapper based on Rust's cargo package manager. Viper is meant to make creating and managing python projects a centralized process.

## Table of Contents
* [Install](#install)
* [How to use](#how-to-use)
    * [Create a new project](#create-a-new-project)
    * [Manage dependencies](#manage-dependencies)
    * [main.py](#main-python-file)
    * [pyproject.toml](#pyproject-toml-file)
* [Commands](#commands)
* [Future Plans](#future-plans)
## Install
You will need the following packages to compile and use this project
- rustc
- cargo
- rustup
- python
- python-pip
- git  
  
The first step is to download the git repository to your machine and move into the downloaded folder
```bash
git clone https://https://github.com/Giona2/Viper.git
cd Viper
```
next, compile the project
```bash
cargo build -r
```
lastly, if you want viper to be accessible through the cli, move the finished binary to the ```/usr/local/bin``` folder and ensure it's included in the ```$PATH``` environment variable
```bash
sudo mv target/release/viper /usr/local/bin/
PATH=$PATH:/usr/local/bin
```
Alternatively, you can also install it locally by putting the binary into the ```$HOME/.local/bin``` directory
```bash
sudo mv target/release/viper /usr/local/bin/
PATH=$PATH:/usr/local/bin
```
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
### Manage dependencies
To start, make sure your pwd (present working directory) is located inside a project created by ```viper new```
```bash
cd /project/directory
```
next, you'll need to open the ```pyproject.toml``` file. You should see a ```[dependencies]``` field
```toml
[dependencies]
required=[]
```
**Tip**: you can search for packages using the ```viper search <package name>```  
Notice the ```required``` field under ```[dependencies]```. This field is the list viper will read and install from.  
All you need to do is find the name of a dependency, we'll take the ```toml``` package for example, and add it to the ```required``` list
```toml
[dependencies]
required=["toml"]
```
then run 
```bash
viper reload
```
to apply the changes. Viper will then install the new packages listed using pip, then add the package's name to an ```installed_packages``` list stored in venv/lib/viper. This will allow you to remove the package just by deleting it's entry from the ```required``` field
```toml
[dependencies]
required=[]
```
and running
```bash
viper reload
```
to delete the dependency and remove it's entry from the installed packages list
### main python file
By default, main.py will be formatted in the entry point format
```python
def main():
    print("hello, world!")


if __name__ == "__main__":
    main()
```
The entry point format is well known for it verbosity, so it was chosen for the default. There are two additional formats you can use, referred to as the class format and the simple format
```python
# class format
class Main:
    def __init__(self):
        print("hello, world!")


if __name__ == "__main__":
    Main()

# simple format
print("hello, world!")
```
To tell viper to use either of these formats, you need to pass in the format tag after your project name in viper run. Assuming the name of the project is first_project, the arguments you can pass in are as follows:
```bash
viper first_project -e | viper first_project --entry-point
viper first_project -c | viper first_project --class
viper first_project -s | viper first_project --simple
```
See below to find out how to install packages to your project
### pyproject toml file
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
The pyproject.py file details the project's metadata. The following fields are meant for these tasks:
- ```[build system]```: Details the machine it must be ran on. Currently, it only details the required programs you need on your computer
- ```[dependencies]```: Details the python packages viper needs to install to run the script. Uses the required field to accomplish this.
- ```[project]```: Details the specific metadata of the project like the project name and version  
  
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
