#![allow(unused_variables)]
extern crate futures;
extern crate tokio_core;
extern crate xrl;

use futures::{future, Future, Stream};
use tokio_core::reactor::Core;
use xrl::{spawn, Client, Frontend, FrontendBuilder, ScrollTo, ServerResult, Style, Update};


// Type that represent our client
struct MyFrontend {
    #[allow(dead_code)]
    client: Client,
}

// Implement how our client handles notifications and requests from the core.
impl Frontend for MyFrontend {
    fn update(&mut self, update: Update) -> ServerResult<()> {
        println!("received `update` from Xi core:\n{:?}", update);
        // note that we could send requests/notifications to the core here with `self.client`
        Box::new(future::ok(()))
    }
    fn scroll_to(&mut self, scroll_to: ScrollTo) -> ServerResult<()> {
        println!("received `scroll_to` from Xi core:\n{:?}", scroll_to);
        Box::new(future::ok(()))
    }
    fn def_style(&mut self, style: Style) -> ServerResult<()> {
        println!("received `def_style` from Xi core:\n{:?}", style);
        Box::new(future::ok(()))
    }
}

struct MyFrontendBuilder;

impl FrontendBuilder<MyFrontend> for MyFrontendBuilder {
    fn build(self, client: Client) -> MyFrontend {
        MyFrontend { client: client }
    }
}

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // spawn Xi core
    let (mut client, core_stderr) = spawn("xi-core", MyFrontendBuilder {}, &handle);

    // start logging Xi core's stderr
    let log_core_errors = core_stderr
        .for_each(|msg| {
            println!("xi-core stderr: {}", msg);
            Ok(())
        })
        .map_err(|_| ());
    core.handle().spawn(log_core_errors);

    // Send a request to open a new view, and print the result
    let open_new_view = client
        .new_view(None)
        .map(|view_name| println!("opened new view: {}", view_name));
    core.run(open_new_view).unwrap();
}
