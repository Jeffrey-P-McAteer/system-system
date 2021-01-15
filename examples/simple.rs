
use system_system::{
  system
};

// Creates a new type MyApp which implements System and Default
system!{
  MyApp,
  java {
    ["/tmp/classes/"],
    "com.example.Main",
  }
}

fn main() {
  let mut app = MyApp::default();

  app.extract();

  app.run();

}
