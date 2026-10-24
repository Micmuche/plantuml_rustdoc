# plantuml_rustdoc

Macro for plantuml graph generation in rust documentation.

## Before using: Linux installation

Install a compatible version of java for plantuml. (cf. http://www.plantuml.com)

Download plantuml.jar and install it in your environment.<br>
For example:

    ~/plantuml/plantuml.jar

Edit a bash file plantuml.sh in the same folder to create a plantuml command:

    #!/usr/bin/bash
    java -jar ~/plantuml/plantuml.jar $@

Add exec writes and link it to your /usr/bin folder to have a plantuml command:

    chmod +x plantuml.sh
    ln -s ~/plantuml/plantuml.sh /usr/bin/plantuml

## How to use

In your Rust file

    /// Toto function
    ///
    #[doc = plantuml_svg!(r#"
    @startuml
    Alice -> Bob : hello
    @enduml
    "#)]
    fn toto() { }