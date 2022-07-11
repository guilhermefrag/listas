struct Lista {
    values: [i32;10],
    ultimo: usize
}

fn criar_lista() -> Lista {
    Lista {
        values: [0;10],
        ultimo: 0
    }
}

fn lista_cheia(lista: &Lista) -> bool {
    if lista.ultimo == 9 {
        return true
    } else {
        return false
    }
}

fn lista_vazia(lista: &Lista) -> bool {
    if lista.ultimo == 0 {
        return true
    } else {
        return false
    }
}

fn insere_lista(lista: &mut Lista, valor: i32) {
    if lista_cheia(lista) {
        println!("Lista cheia");
    } else {
        lista.values[lista.ultimo] = valor;
        lista.ultimo += 1;
    }
}



fn main() {
    
    
}
