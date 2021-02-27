
use system_system::{
  system
};

/**
# This shellcode creates the /tmp/classes/ folder.
# For real projects this would be something like
# build/classes/ from a gradle build/ directory.

mkdir -p /tmp/classes/
cat <<EOF > /tmp/classes/Main.java
public class Main {
  public static void main(String... args) {
    System.out.printf("Java version %s%n", System.getProperty("java.version"));
  }
}
EOF
( cd /tmp/classes ; javac Main.java )

*/

// Creates a new type MyApp which implements System and Default
system!{
  MyApp,
  java {
    ["/tmp/classes/"],
    "Main",
  }
}

fn main() {
  let mut app = MyApp::default();

  app.extract();

  app.run();

}
