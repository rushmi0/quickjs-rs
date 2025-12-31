// Rust Design Patterns: Typestate
struct File<State> {
    state: State,
}

#[derive(Debug)]
struct Closed;
struct Open;

impl File<Closed> {
    fn open(self) -> File<Open> {
        println!("Opening the file.");
        File { state: Open }
    }
}


impl File<Open> {
    fn read(&self) {
        println!("Reading the file.");
    }

    fn write(&self) {
        println!("Writing to the file.");
    }

    fn close(self) -> File<Closed> {
        println!("Closing the file.");
        File { state: Closed }
    }
}


fn main() {
    let closed_file = File { state: Closed };

    let open_file = closed_file.open();
    open_file.read();
    open_file.write();

    let closed_again = open_file.close();
    let a = closed_again.state;
    println!("{:?}", a)

}