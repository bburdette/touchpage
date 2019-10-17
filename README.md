# touchpage

Configurable web touch controls together with a built in webserver.

The idea is to have a simple way to configure a set of touch enabled buttons, sliders, and labels, and present them on a web page.  When the user manipulates the controls, websocket messages are sent to the server, which updates internal state, updates all clients with the new changes, and passes the events to your rust program.  On the rust side, you can update the control state, and web users will see the state changes.

I've written a few apps that use the lib.

First is the [example](https://github.com/bburdette/touchpage/tree/master/example) app in this project.  Copy and paste to get started on your own project. 

[mousepage](https://github.com/bburdette/mousepage) - start the server on your computer, navigate to the page with your phone, bam your phone is now controlling your computer mouse.

[oscpad](https://github.com/bburdette/oscpad) - A standalone app that control updates into OSC messages and vice versa.  [Overview/tutorial](https://github.com/bburdette/oscpad/wiki/Get-started-with-oscpad)  

### If you want to hack on the elm code

The elm code is already compiled into js thats baked in to the rust library, so there's no need to compile the elm unless you want to make changes to it.  If you do decide to work on the elm code, the elm build requires ambr, part of the amber cargo package.  Install that with 'cargo install amber'.

From the project directory, use ./build-elm.sh, which will build the elm and make an index.html to use in the example project.  In the example you'll have to change the main.rs a little to use the html file.  Compile the example and run it with ./runit.sh.  So to sum up:

    0) cargo install amber
    1) cd elm/
    2) ./build-elm.sh
    3) cd ../example
    4) cargo build
    5) ./runit.sh
