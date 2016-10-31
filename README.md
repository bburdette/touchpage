# touchpage
Configurable web controls with shared state.  [Overview/tutorial here](https://github.com/bburdette/oscpad/wiki/Get-started-with-oscpad)  

The idea is to have a simple way to configure a set of touch enabled buttons, sliders, and labels, which are presented on a web page.  When the user manipulates the controls, websocket messages are sent to the server, which updates internal state, updates all clients with the new changes, and passes the events to your rust program.  The library user can also change the control state, which is reflected in the clients.  

### Some notes on elm compiling.

The elm build requires ambr, part of the amber cargo package.  Install that with cargo install amber.

From the project directory, use ./build-elm.sh to build the elm and merge the js into stringDefaults.rs.  Then do cargo build to get the rust server.  Run the rust server with ./runit.sh.  So to sum up:

    0) cargo install amber
    1) ./build-elm.sh
    2) cargo build
    3) ./runit.sh
