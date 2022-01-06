import os, requests, sys
from dataclasses import dataclass
from bs4 import BeautifulSoup
from typing import Optional

logo = """
____    ____   _______. __    __  
\   \  /   /  /       ||  |  |  | 
 \   \/   /  |   (----`|  |__|  | 
  \      /    \   \    |   __   | 
   \    / .----)   |   |  |  |  | 
    \__/  |_______/    |__|  |__| 
"""

def get_input():
    print("What do you want to do?: ")
    print("1. Install Binary")
    print("2. Build Locally from main branch")
    print("3. Update latest vsh version")
    while True:
        x = input("> ")
        try:
            val = int(x)
            if val not in [1, 2 , 3]:
                print("Please enter correct Input")
                continue
            return val
        except ValueError:
            print("Choice must be a number!")

@dataclass
class Release:
    num: str
    message: str

def find_releases():
    page = requests.get("https://github.com/Vaimer9/vsh/releases")
    soup = BeautifulSoup(page.content, 'html.parser')
    to_return = []
    for box in soup.find_all('div', class_="Box-body"):
        num = box.findChildren("a", class_="Link--primary")[0].get('href').split("/tag/")[1]
        message = next(box.findChildren("div", class_="markdown-body my-3")[0].children).string
        to_return.append(Release(num=num, message=message))
    return to_return


def get_version(releases):
    print("Which version do you want to install?")
    print("Example: vX.X.XX")
    print("`l` for latest")
    print("`q` to display what releases are available")
    while True:
        x = input("> ")
        if x.lower() == 'l':
            return 'l'
        if releases is not None:
            if x.lower() == 'q':
                for release in releases:
                    print(f"Release {release.num}: \n\t\t \"{release.message}\"")
                continue
            else:
                if x not in [rel.num for rel in releases]:
                    print("Invalid release number!")
                    continue
                return x
        else:
            return x

def check_for_rustc():
    print("Do you have rust compiler installed or not?: (y, n)")
    while True:
        x = input("> ")
        if x.lower() == "y":
            return False
        elif x.lower()  == "n":
            return True
        else:
            print("Please enter correct value")
            continue

def get_path(installed):
    if installed:
        print("Where have you installed vsh?")
        print("enter `b` if  you have it in /bin directory")
        path = input("> ")

        if os.path.isfile(f"{path}") and os.path.isfile(f"{path}/vsh") is False:
            print("That is the wrong directory!")
        return path
    else:
        print("Where do you want to install the vsh binary?")
        print("This is where is will be updated")
        print("Enter `b` is its in /bin directory")
        path = input("> ")
        

def install_default_config_file():
    print("Do you want to automatically install the default configuration file?: (y, n)")
    while True:
        x = input("> ")
        if x.lower() == "y":
            break
        elif x.lower()  == "n":
            return
        else:
            print("Please enter correct value")
            continue
    os.system(f"cp ./default-config {os.environ['HOME']}/.vshrc.json")

def main():
    print(logo)
    x = get_input()
    
    releases = None
    print("Getting current releases")
    try:
        releases = find_releases()
    except Exception:
        print("Could not get current releases!", file=sys.stderr)

    if x == 1:
        x = get_version(releases)
        print("Installing binary in current Directory")
        if x == "l":
            os.system("wget https://github.com/Vaimer9/vsh/releases/latest/download/vsh -P /bin")
            os.system("sudo chmod +x /bin/vsh")
        else:
            ret_val = os.system(f'wget https://github.com/Vaimer9/vsh/releases/download/{x}/vsh -P /bin')
            if ret_val != 0:
                print("Could not download vsh binary!", file=sys.stderr)
                sys.exit(1)
            os.system("sudo chmod +x vsh")

    elif x == 2:
        
        if check_for_rustc():
            print("Installing Rustc")
            os.system("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
            print("Rustc Installed")

        print("Cloning Repository")
        os.system("git clone https://github.com/Vaimer9/vsh")
        os.system("cd vsh")
        os.system("cargo build --release")
        print("The Binary is created inside target/release directory")
    elif x == 3:
        install_default_config_file()

if __name__ == '__main__':
    try:
       main()
    except KeyboardInterrupt:
        print("Closed") 
