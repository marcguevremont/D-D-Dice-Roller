# D&D Dice Roller
Roll like a master in your next Dungeons & Dragons game.

### How to use
Type roll on the terminal and follow the instructions.

```
USAGE:
    roll [FLAGS]

FLAGS:
    -a, --att        Attack rolls
    -d, --dmg        Damage rolls
    -h, --help       Prints help information
    -V, --version    Prints version information
```

### To install on mac
Download the binary on the release page.
Open your terminal and go to your Downloads folder 
```
cd ~/Downloads
```
Change the binary to an executable 
```
chmod u+x mac-os-64-bit-roll
```
Create a folder and Copy the file to an other place 
```
mkdir ~/Documents/d-d-dice-roller
mv mac-os-64-bit-roll ~/Documents/d-d-dice-roller/roll
```
Optionally add the program to your path.
Open nano and add the path to your .bash_profile
```
nano ~/.bash_profile 
export PATH="~/Documents/d-d-dice-roller:$PATH"
```
Restart the terminal or type
```
source ~/.bash_profile
```
You can type roll everywhere on the terminal to start rollin'

**Enjoy!**
