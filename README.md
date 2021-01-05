# Velox

## Introduction
Velox is a framework that focuses on simplicity, performance and ease of use. It allows you to build cross platform native apps using web-technology. Velox uses webview and rust under the hood which helps to keep the binary size to absolute minimal and increasing the performance.

## Performance
*Todo*

## Features
*Todo*

## Installation
### Install Rust
You can easily install rust using [rustup](https://rustup.rs/)  which is the official installer for rust programming language. This will also install all the other tools which will help you in your development, e.g- cargo, clippy, etc.

### Install Velox-CLI
You can use cargo to install velox-cli. Cargo would be automatically installed if you used rustup to install rust. Run
  `cargo install velox` in your terminal, this will install velox-cli globally in your machine then to check if velox was successfully installed run `velox --version` in your terminal and this should display a version number.

**Note:**  Velox uses [webview](https://github.com/webview/webview) under the hood which reqires gtk-webkit2 on linux. To install gtk-webkit2, run `sudo apt install webkit2gtk-4.0` in your terminal.


## Quick Example
In this quick example we will create a simple counter app which will increase a countdown when a button is pressed.

### Setup
1. Run `velox new project_name` in our terminal this will create a new project in your current working directory.
2. Now `cd` into this new directory and go to `web` folder.

### Actual coding
Now when we have finally finished setting up everything, we can begin writing our app.
1. Open the `your_project_name/web/index.html` file in your favorite text editor and put the following code inside the `<body>` tag.

```html
    <body>
      <div id=counter>
        <btn id="increase">increase counter<btn>
        <h4>0<h4>
      </div>
    </body>
```
2.  Now open the `your_project_name/web/src/index.js` file and put the following code inside it.

```javascript
function increaseCounter() {
    let count = parseInt(document.querySelector("span").innerText);
    count += 1;
    document.querySelector("span").innerText = count;
}

document.getElementById("increase").addEventListener("click", increaseCounter);
```
3. You have now written all the frontend code for your app. Now we just have to tell velox to create an app window and display our frontend code. To do this open the `your_project_name/src/main.rs` file and put the following code inside the main function.

```rust
fn main() {
    let app = AppBuilder::new().build(); // build a new app window
    app.run() // run the app
}
```

4. We have finished the coding part. Now to run your app, go to the root of your project and run `velox run` command.

If everything went fine then you should see a new app window which will display a button and a counter next to it.

### Building your project
Building a project is super simple, just go to the root of your project and run `velox build` command. This will create a new binary file in `dist/` directory.
**Note:** By default velox will build your project in debug mode. This is good when you're developing your app but when you want to release it make sure to build your app in release mode you can do it by using 
`velox build --release` command and velox will take care of optimising your code.

## Comparison

Todo

## FAQ's

Todo