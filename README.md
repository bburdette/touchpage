# touchpage

#### I'm splitting the oscpad repo into two parts, the OSC stuff and the webserver-UI part.  This is the webserver-UI part, in library form, so that its easy to make standalone rust programs with oscpad style UIs.

Configurable web controls with shared state.  [Overview/tutorial here](https://github.com/bburdette/oscpad/wiki/Get-started-with-oscpad)  

The idea is to have a simple way to configure a set of touch enabled buttons, sliders, and labels, which are presented on a web page.  When the user manipulates the controls, websocket messages are sent to the server, which updates internal state, updates all clients with the new changes, and passes the events to your rust program.  The library user can also change the control state, which is reflected in the clients.  

As of 0.2.0, touch controls are configurable from rust, instead of having to edit an undocumented json file.  Check the example folder for that, and other usage details.

### Some notes on elm compiling.

The elm code is already compiled into js, so there's no need to compile just to use it.  But if you do want to change the elm code, the elm build requires ambr, part of the amber cargo package.  Install that with 'cargo install amber'.

From the project directory, use ./build-elm.sh to build the elm and make an index.html to use in the example project - see the example code for details.  Run the rust server with ./runit.sh.  So to sum up:

    0) cargo install amber
    1) cd elm/
    2) ./build-elm.sh
    3) cd ../example
    4) cargo build
    5) ./runit.sh
