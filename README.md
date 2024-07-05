
# Project Name
GamePad/Controller
## Description

Simple GamePad built on a breadboard to test in games. Is built on Windows XInput API, so it will detect it as an XBOX Controller. The purpose of this project is (besides building a controller) to test input latency in different games and on different systems, computers and monitors in order to see the difference in input latency. Will also compare with an actual Xbox Series controller.

## Hardware

<!-- Fill out this table with all the hardware components that you mght need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Rapspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| BreadBoard | Build the circuit | [10 RON] https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/8-breadboard-830-points.html |
| Buttons ( 12-15 ) | Input for controller | [5 RON] https://www.optimusdigital.ro/ro/butoane-i-comutatoare/1119-buton-6x6x6.html?search_query=buton&results=222 |
| Resistances | To not fry the circuit | [5 RON] https://www.optimusdigital.ro/ro/componente-electronice-rezistoare/858-rezistor-025w-18k.html?search_query=rezistor&results=120 |
| 2x JoySticks | Move and aim with the controller | [10 RON] https://www.optimusdigital.ro/ro/senzori-senzori-de-atingere/742-modul-joystick-ps2-biaxial-negru-cu-5-pini.html?search_query=joystick&results=42 |

## Links

<!-- Add a few links that got you the idea and that you think you will use for your project -->

1. [link](https://learn.microsoft.com/en-us/windows/win32/xinput/getting-started-with-xinput)
2. [link](https://www.howtogeek.com/792984/directinput-vs.-xinput-for-game-controllers-whats-the-difference/)
...

# Software and Code

### Rust Code for Raspberry Pi Pico W
The Rust code deployed on the Raspberry Pi Pico W reads button presses and joystick movements, packaging them into a structured format to be sent over UDP.

## Run the code
* Open the lab-main-Copy-Copy file in which IDE you desire (I used RustRover for this project)
* The code is located in /lab08/lab08_ex3_4/src/main.rs

⚠️ **Warning:** Ensure change the IP adresses and the path target to the cyw43-firmware in the code before you build it. Also the ID and the password of the network. Make sure the band of the network is 2.4GHZ!

```rust
let fw = include_bytes!("C:/Users/Andrei/Desktop/lab-main/cyw43-firmware/43439A0.bin");
let clm = include_bytes!("C:/Users/Andrei/Desktop/lab-main/cyw43-firmware/43439A0_clm.bin");

```

```rust
//change the ip adress as well



match socket
            .send_to(
                &packet,
                //here
                IpEndpoint::new(IpAddress::v4(192, 555, 553, 555), 1234), 
            )
            .await
        {
            Ok(()) => {
                info!("sent")
            }
            Err(e) => {
                warn!("send error: {:?}", e);
            }
        }
        info!("Sending packet to UDP server...");

        //match for joystick
        match socket.send_to(&packet, IpEndpoint::new
        //and here
        (IpAddress::v4(192, 555, 555, 555), 1234)).await {
            Ok(()) => info!("Joystick data sent"),
            Err(e) => warn!("send error: {:?}", e),
         }

    }
```
```rust
//change the network

const WIFI_NETWORK: &str = "da";
const WIFI_PASSWORD: &str = "da";
```


## Building the code
* Open a terminal and make sure you are in the correct directory 

⚠️ **Warning:** Please double check if the terminal correctly detects the directory (including the -Copy - Copy sufixes of the name), otherwise it won't build correctly

* To make sure you are in the correct file, use these commands

```
cd '.\lab-main - Copy - Copy\lab08\lab08_ex3_4\
```
* To build the project, simply type in the terminal
```
cargo build --release --target=thumbv6m-none-eabi  
```
* When it compiles, we need to open an UDP server so that we can send the data.
* Open the _udpserver_ file as a **separate** project/tab.
* Run the server with _cargo run_
* Once the server is fully loaded, go back to the 1st project and type in the terminal in order to flash it on the Pico

```
elf2uf2-rs -d -s ..\..\target\thumbv6m-none-eabi\release\lab08_ex3_4

```
* When the Pico connects to the UDP server, it will constantly send packets to the server
* Now we need a "translator" to decode the packets

# Detecting the Controller on Windows

## Prerequisites

Before you begin setting up the project, ensure you have the following prerequisites installed on your system:

## System Requirements
- **Operating System**: Windows 10 or newer is recommended for full compatibility.

## Software Requirements
- **.NET Framework**: .NET 5.0 or later. Make sure to download and install it from [Microsoft's official site](https://dotnet.microsoft.com/download).
- **Visual Studio** or **Visual Studio Code**: For code editing and debugging. Visual Studio can be downloaded from [here](https://visualstudio.microsoft.com/downloads/).

## Driver Installation
- **ViGEmBus Driver**: This driver is required to create a virtual Xbox controller that the system recognizes.
  - Download the latest version of the ViGEmBus driver from the [ViGEmBus GitHub releases page](https://github.com/ViGEm/ViGEmBus/releases).
  - Follow the installation instructions provided there to install the driver on your system.


## Additional Libraries
- **Nefarius.ViGEm.Client**: This is a .NET library used to interact with the ViGEmBus driver programmatically.
  - Ensure your project references this library. It can be added via NuGet in Visual Studio or by running the following command in your project directory:
    ```bash
    dotnet add package Nefarius.ViGEm.Client
    ```

## Hardware Requirements
- If testing the project physically, ensure you have a compatible hardware setup, including a game controller or custom hardware interfacing with your PC.

Make sure all prerequisites are properly installed and configured before proceeding with the project setup.



# Opening the Project and Adding NuGet Packages

## Opening the C# Project in Visual Studio

To start working on the project, you need to open it in an Integrated Development Environment (IDE). Visual Studio is recommended for C# projects. Follow these steps to open the project:

1. **Launch Visual Studio**:
   - Open Visual Studio. If you haven't installed it yet, download it from [Visual Studio Downloads](https://visualstudio.microsoft.com/downloads/).

2. **Open Project or Solution**:
- Name of the C# code is _ConsoleApp1_
   - On the initial launch screen, click **Open a project or solution**.
   - Navigate to the folder where you have cloned or downloaded the project.
   - Select the `.sln` or `.csproj` file and click **Open** to load the project into Visual Studio.

## Adding a NuGet Package

NuGet is a package manager for .NET that enables developers to share and consume useful code. Adding a NuGet package to your project can extend its functionality easily. Here's how to add a NuGet package:

1. **Open NuGet Package Manager**:
   - Right-click on the project in the **Solution Explorer** in Visual Studio.
   - Click **Manage NuGet Packages**.

2. **Browse for Packages**:
   - Go to the **Browse** tab in the NuGet Package Manager.
   - Type the package name you want to install in the search box (e.g., `Nefarius.ViGEm.Client`).

3. **Install the Package**:
   - Select the package from the list of search results.
   - Click on the **Install** button next to the package details.
   - Review any license agreements, if prompted, and accept them to proceed with the installation.

4. **Verify Package Installation**:
   - After installation, the package will appear under the **Dependencies** > **NuGet** node in your project structure.
   - You can now use the functionalities provided by the NuGet package in your project code.

## Verifying the Setup

After opening the project and adding the necessary NuGet packages, build the project to verify that everything is set up correctly:

- Click on **Build** > **Build Solution** in the menu bar.
- Ensure that the build completes without errors.

Before running the code, make sure to clode **ONLY** the UDP server in Rust, as C# will throw an exception.

# Mapping the buttons
* Run the C# code and it will display all the "translated" packages that the PICO is sending. 
## Step 1: Connect Your USB Controller

First, you need to physically connect your USB controller to your Windows machine.

- **Plug the USB connector** of your controller into a free USB port on your computer.
- Wait for Windows to recognize the device and install any necessary drivers. This process is typically automatic.

## Step 2: Check Device Recognition

After connecting your controller, verify that Windows has recognized it.

- Open **Control Panel** by typing `Control Panel` in the Start menu and pressing Enter.
- Navigate to **Hardware and Sound** > **Devices and Printers**.
- Under **Devices**, look for your USB controller. It should be listed under "Devices" if Windows recognizes it properly.
- If Device Manager can't find it, search in the Start menu for "Set Up USB Controllers".

## Step 3: Calibrate Your Controller

To ensure optimal performance, you might need to calibrate your controller. Here’s how to do it:

- Right-click on your controller icon in **Devices and Printers** and select **Game controller settings**.
- In the Game Controllers window, select your controller and click **Properties**.
- In the Properties window, go to the **Settings** tab and click on **Calibrate**.
- Follow the on-screen instructions to complete the calibration process.

## Step 4: Test Your Controller

After calibration, it’s a good idea to test the controller to make sure all buttons and joysticks are working as expected.

- In the Properties window of your controller, switch to the **Test** tab.
- Test each button and joystick on your controller. As you press buttons and move joysticks, corresponding buttons should light up in the test screen, confirming their functionality.

That's pretty much it. Let me know if there are questions regarding the compilation process.







