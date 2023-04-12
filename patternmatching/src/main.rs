fn main() {
    let something = Some(5);

    match something {
        Some(x) => println!("{}", x),
        None => {}
    }

    if let Some(x) = something {
        println!("{}", x);
    }

    let result = Some(10);

    let value = if let Some(x) = result {
        // comentário só pra forçar o formatter a manter em multiplas linhas
        x + 5
    } else {
        0
    };

    println!("{}", value);
}
