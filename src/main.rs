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

fn imprime_lista(lista: &mut Lista) {
    if lista_vazia(lista) {
        println!("Lista vazia");
    } else {
        for i in 0..lista.ultimo {
            println!("{}", lista.values[i]);
        }
    }
}

fn insere_inicio(lista: &mut Lista, valor: i32) {
    if lista_cheia(lista) {
        println!("Lista cheia");
    } else {
        for i in (0..lista.ultimo).rev() {
            lista.values[i+1] = lista.values[i];
        }
        lista.values[0] = valor;
        lista.ultimo += 1;
    }
}

fn insere_final(lista: &mut Lista, valor: i32) {
    if lista_cheia(lista) {
        println!("Lista cheia");
    } else {
        lista.values[lista.ultimo] = valor;
        lista.ultimo += 1;
    }
}

fn ordena_asc(lista: &mut Lista) {
    if lista_vazia(lista) {
        println!("Lista vazia");
    } else {
        for i in 0..lista.ultimo {
            for j in 0..lista.ultimo {
                if lista.values[i] < lista.values[j] {
                    let aux = lista.values[i];
                    lista.values[i] = lista.values[j];
                    lista.values[j] = aux;
                }
            }
        }
    }
}

fn ordena_desc(lista: &mut Lista) {
    if lista_vazia(lista) {
        println!("Lista vazia");
    } else {
        for i in 0..lista.ultimo {
            for j in 0..lista.ultimo {
                if lista.values[i] > lista.values[j] {
                    let aux = lista.values[i];
                    lista.values[i] = lista.values[j];
                    lista.values[j] = aux;
                }
            }
        }
    }
}


fn main() {
    
    let mut list: Lista = criar_lista();
    let list_ref = &mut list;
    insere_lista(list_ref, 1);
    insere_lista(list_ref, 2);
    insere_inicio(list_ref, 3);
    insere_final(list_ref, 4);
    ordena_asc(list_ref);
    ordena_desc(list_ref);
    imprime_lista(list_ref);
    
}
