import os 

def get_input():
    print("What do you want to do?: ")
    print("1. Install Binary")
    print("2. Build Locally from main branch")
    x = input("> ")
    return int(x)

def get_version():
    print("Which version do you want to install?")
    print("Example: vX.X.XX")
    print("`l` for latest")
    x = input("> ")
    return x

def check_for_rustc():
    print("Do you have rust compiler installed or not?: (y, n)")
    x = input("> ")
    if x == "y":
        return True
    elif x  =="n":
        return False
    else:
        print("Please enter correct value")
        check_for_rustc()

def main():
    if get_input() == 1:
        x = get_version()
        print("Installing binary in current Directory")
        if x == "l":
            os.system("wget https://github.com/Vaimer9/vsh/releases/latest/download/vsh")
            os.system("chmod +x vsh")
            os.system("./vsh")
        else:
            os.system(f'wget https://github.com/Vaimer9/vsh/releases/download/{x}/vsh')
            os.system("chmod +x vsh")
            os.system("./vsh")

    elif get_input == 2:
        if check_for_rustc():
            print("Installing Rustc")
            os.system("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
            print("Rustc Installed")

            print("Cloning Repository")
            os.system("git clone https://github.com/Vaimer9/vsh")
        else:
            print("Cloning Repository")
            os.system("git clone https://github.com/Vaimer9/vsh")


if __name__ == '__main__':
    try:
        main()
    except KeyboardInterrupt:
        print("Closed")
