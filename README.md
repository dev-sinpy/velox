# Velox [WIP]
> Please ignore this repo for now. It is still under heavy development and most of the stuff doesn't work. Contact [me](mailto::dev.sinpy@protonmail.com) if you have any other questions.

## Introduction
A GUI framework that focuses on simplicity, performance and ease of use. It allows developers to build cross platform native apps using their favourite web frameworks. Velox uses [wry](https://github.com/tauri-apps/wry) and rust under the hood, which helps to keep the binary size to absolute minimal without loosing much performance.

## Performance
*Todo*

## Features
*Todo*

## Setup
### Install Rust
You can easily install rust using [rustup](https://rustup.rs/)  which is the official installer for rust programming language. This will also install all the other tools which will help you in your development, e.g- cargo, clippy, etc.

### Install Velox-CLI
You can use cargo to install velox-cli. Run `cargo install velox` in your terminal, this will install velox-cli globally in your machine then to check if velox was successfully installed run `velox --version` in your terminal and this should display a version number.

**Note:**  Before going further, wry requires some other dependencies to be installed, so please head over to their github [repo](https://github.com/tauri-apps/wry) to find out what packages you have to install according to your operating system.


## Quick Example
In this quick example, we will create a simple counter app which will increase a countdown when a button is pressed.

### Setup
1. Run `velox new counter_app` in our terminal this will create a new velox project in your current working directory.
2. Now `cd` into this new directory and go to `web` folder.

### Actual coding
Now when we have finally finished setting up everything, we can begin writing our app.
1. Open `counter_app/web/index.html` file in your favorite text editor and put the following code inside the `<body>` tag.

```html
    <body>
      <div id=counter>
        <btn id="increase">increase counter<btn>
        <h4>0<h4>
      </div>
    </body>
```
2.  Now open `counter_app/web/src/index.js` file and put the following code inside it.

```javascript
// Increments counter by one at each click
function increaseCounter() {
    let count = parseInt(document.querySelector("span").innerText); // Current counter number
    count += 1; // Increment count
    document.querySelector("span").innerText = count; // updates value of the counter
}
// Listen for click event
document.getElementById("increase").addEventListener("click", increaseCounter);
```
3. You have now written all the frontend code for your app. Now we just have to tell velox to create an app window and display our frontend code. To do this open the `counter_app/src/main.rs` file and put the following code inside the main function.

```rust
use velox::AppBuilder;

fn main() {
    // Builds an app window with all the attributes pulled from config file
    let app = AppBuilder::from_config(include_str!("../velox.conf.json").to_string())
      .build();
    app.run().unwrap(); // starts a new event loop and runs the app until completion
}

```

4. We have finished the coding part. Now to run your app, go to the root of your project and run `velox run` command.

If everything went fine then you should see a new app window which will display a button and a counter next to it.

### Building your project
TO build a project, run `velox build` command inside your project directory. This will create a new binary file in `dist/` directory.
**Note:** By default velox will build your project in debug mode. This is good when you're developing your app but when you want to release it make sure to build your app in release mode you can do it by using 
`velox build --release` command and velox will take care of optimising your code.

## Comparison

Todo

## FAQ's

Todo